use std::{collections::HashMap, path::Path, vec};

use dotenvy::var;
use regex::Regex;

use super::{
    middleware::{self, Middleware},
    request::Request,
    response::{self, Response},
    HTTPMethod,
};

fn strip_braces(s: &str) -> &str {
    s.strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or(s)
}

pub struct Router {
    pub routes: Box<RouteTree>,
    pub middlewares: HashMap<String, Middleware>,
    prefix: String,
}
#[derive(Debug)]
pub struct RouteTree {
    path: String,
    handlers: HashMap<HTTPMethod, RouteHandler>,
    children: Vec<Box<RouteTree>>,
    middlewares: Vec<String>,
}

fn merge_trees(target: &mut RouteTree, source: RouteTree) {
    for (method, handler) in source.handlers {
        target.handlers.insert(method, handler);
    }

    for middleware in source.middlewares {
        if !target.middlewares.contains(&middleware) {
            target.middlewares.push(middleware);
        }
    }

    for mut source_child in source.children {
        if let Some(target_child) = target
            .children
            .iter_mut()
            .find(|child| child.path == source_child.path)
        {
            merge_trees(target_child, *source_child);
        } else {
            target.children.push(source_child);
        }
    }
}

impl RouteTree {
    pub fn extend(&mut self, tree: Box<RouteTree>) {
        merge_trees(self, *tree);
    }
}

pub type RouteHandler =
    fn(request: Request, response: Response, route_values: HashMap<String, String>);

impl Router {
    pub fn new() -> Router {
        Router {
            middlewares: HashMap::new(),
            prefix: String::new(),
            routes: Box::new(RouteTree {
                path: String::new(),
                handlers: HashMap::new(),
                children: vec![],
                middlewares: vec![],
            }),
        }
    }

    pub fn new_with_prefix<T: Into<String>>(prefix: T) -> Self {
        Router {
            middlewares: HashMap::new(),
            prefix: prefix.into(),
            routes: Box::new(RouteTree {
                path: String::new(),
                handlers: HashMap::new(),
                children: vec![],
                middlewares: vec![],
            }),
        }
    }

    pub fn get(&mut self, path: &str, handler: RouteHandler, middlewares: Option<Vec<String>>) {
        self.register(
            path,
            HTTPMethod::GET,
            handler,
            middlewares.unwrap_or_default(),
        );
    }
    pub fn post(&mut self, path: &str, handler: RouteHandler, middlewares: Option<Vec<String>>) {
        self.register(
            path,
            HTTPMethod::POST,
            handler,
            middlewares.unwrap_or_default(),
        );
    }
    pub fn delete(&mut self, path: &str, handler: RouteHandler, middlewares: Option<Vec<String>>) {
        self.register(
            path,
            HTTPMethod::DELETE,
            handler,
            middlewares.unwrap_or_default(),
        );
    }
    pub fn put(&mut self, path: &str, handler: RouteHandler, middlewares: Option<Vec<String>>) {
        self.register(
            path,
            HTTPMethod::PUT,
            handler,
            middlewares.unwrap_or_default(),
        );
    }
    pub fn patch(&mut self, path: &str, handler: RouteHandler, middlewares: Option<Vec<String>>) {
        self.register(
            path,
            HTTPMethod::PATCH,
            handler,
            middlewares.unwrap_or_default(),
        );
    }

    pub fn group<T: AsRef<str>, F: FnOnce(&mut Router)>(&mut self, path: T, configure: F) {
        let prefix = path.as_ref().to_string();
        let mut sub_router = Router::new_with_prefix(prefix);
        configure(&mut sub_router);
        self.routes.extend(sub_router.routes);
    }

    pub fn register(
        &mut self,
        path: &str,
        method: HTTPMethod,
        handler: RouteHandler,
        middlewares: Vec<String>,
    ) {
        let prefix = self.prefix.trim_matches('/');
        let clean_path = path.trim_matches('/');
        let path = format!(
            "{}{}{}",
            prefix,
            if prefix.len() > 0 && clean_path.len() > 0 {
                "/"
            } else {
                ""
            },
            clean_path
        );
        let mut route_path = &mut self.routes;
        if path.len() > 0 {
            for dir in path.split('/') {
                let existing_index = route_path
                    .children
                    .iter()
                    .position(|child| child.path == dir);

                if let Some(index) = existing_index {
                    route_path = route_path.children.get_mut(index).unwrap();
                } else {
                    route_path.children.push(Box::new(RouteTree {
                        path: dir.to_string(),
                        children: vec![],
                        middlewares: middlewares.clone(),
                        handlers: HashMap::new(),
                    }));

                    route_path = route_path.children.last_mut().unwrap();
                }
            }
        }
        route_path.handlers.insert(method, handler);
    }

    pub fn register_middleware<T: Into<String>>(&mut self, name: T, handler: Middleware) {
        self.middlewares.insert(name.into(), handler);
    }

    pub fn invoke(&self, request: Request, mut response: Response) {
        let path = request.path.trim_matches('/');

        let mut current_path = &self.routes;
        let mut found = true;
        let mut was_wildcard = false;
        let mut dyn_route_params: HashMap<String, String> = HashMap::new();

        if path.len() > 0 {
            let param_reg = Regex::new(r"\{(\w+)\}").unwrap();
            for dir in path.split('/') {
                let existing_index = current_path.children.iter().position(|child| {
                    child.path == dir || child.path == "*" || param_reg.is_match(&child.path)
                });

                if let Some(index) = existing_index {
                    current_path = current_path.children.get(index).unwrap();
                    if current_path.path == "*" {
                        was_wildcard = true
                    } else if param_reg.is_match(&current_path.path) {
                        dyn_route_params
                            .insert(strip_braces(&current_path.path).to_owned(), dir.to_owned());
                    }
                } else {
                    if !was_wildcard {
                        found = false;
                    }
                    break;
                }
            }
        }

        if found && !current_path.handlers.contains_key(&request.method) {
            let has_wildcard = current_path
                .children
                .iter()
                .position(|child| child.path == "*");

            if let Some(index) = has_wildcard {
                current_path = current_path.children.get(index).unwrap()
            }
        }

        if found && (current_path.handlers.contains_key(&request.method)) {
            for middleware in &current_path.middlewares {
                if let Some(handler) = self.middlewares.get(middleware) {
                    handler(&request, &mut response, &dyn_route_params);
                }
            }
            current_path.handlers.get(&request.method).unwrap()(
                request,
                response,
                dyn_route_params,
            );
        } else {
            self.try_serve_public(request, response);
        }
    }

    fn try_serve_public(&self, request: Request, mut response: Response) {
        let mut public_path = var("APP_PUBLIC_DIR")
            .unwrap_or("public".to_string())
            .trim_matches('/')
            .to_string();
        public_path.push_str(&request.path);
        response.file(Path::new(&public_path));
    }
}

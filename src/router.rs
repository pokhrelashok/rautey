use std::{
    collections::{HashMap, HashSet},
    path::Path,
    vec,
};

use dotenvy::var;
use regex::Regex;

use crate::middleware::session_handler;

use super::{middleware::Middleware, request::Request, response::Response, HTTPMethod};

fn strip_braces(s: &str) -> &str {
    s.strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or(s)
}

pub struct Router {
    pub routes: Box<RouteTree>,
    registered_middlewares: HashMap<String, Middleware>,
    middlewares: HashSet<String>,
    prefix: String,
}
#[derive(Debug)]
pub struct RouteTree {
    path: String,
    handlers: HashMap<HTTPMethod, RouteHandler>,
    children: Vec<Box<RouteTree>>,
    middlewares: HashSet<String>,
}

fn merge_trees(target: &mut RouteTree, source: RouteTree) {
    for source_child in source.children {
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
    #[must_use]
    pub fn new() -> Router {
        let mut registered_middlewares: HashMap<String, Middleware> = HashMap::new();
        let session_driver = var("SESSION_DRIVER").unwrap_or_default();
        if session_driver == "cookie" || session_driver == "cache" {
            registered_middlewares.insert("session".to_string(), session_handler);
        }
        let mut middlewares = HashSet::new();
        middlewares.insert("session".to_string());

        Router {
            registered_middlewares,
            prefix: String::new(),
            middlewares,
            routes: Box::new(RouteTree {
                path: String::new(),
                handlers: HashMap::new(),
                children: vec![],
                middlewares: HashSet::new(),
            }),
        }
    }

    pub fn with_prefix<T: Into<String>>(&mut self, prefix: T) -> &mut Self {
        self.prefix = prefix.into();
        self
    }

    pub fn with_middlewares(&mut self, middlewares: HashSet<String>) -> &mut Self {
        self.middlewares = middlewares.iter().cloned().collect();
        self
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

    pub fn group<T: AsRef<str>, F: FnOnce(&mut Router)>(
        &mut self,
        path: T,
        middlewares: Option<&[String]>,
        configure: F,
    ) {
        let prefix = format!("{}/{}", self.prefix, path.as_ref())
            .trim_matches('/')
            .to_owned();
        let mut sub_router = Router::new();
        let group_middlewares: HashSet<String> =
            middlewares.map_or(HashSet::new(), |f| f.iter().cloned().collect());
        let mut middlewares = self.middlewares.clone();
        middlewares.extend(group_middlewares);
        sub_router.with_prefix(prefix).with_middlewares(middlewares);
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
                if dir.is_empty() {
                    continue;
                }
                let existing_index = route_path
                    .children
                    .iter()
                    .position(|child| child.path == dir);

                if let Some(index) = existing_index {
                    route_path = route_path.children.get_mut(index).unwrap();
                } else {
                    let mut all_middlewares = self.middlewares.clone();
                    all_middlewares.extend(middlewares.iter().cloned());
                    route_path.children.push(Box::new(RouteTree {
                        path: dir.to_string(),
                        children: vec![],
                        middlewares: all_middlewares,
                        handlers: HashMap::new(),
                    }));

                    route_path = route_path.children.last_mut().unwrap();
                }
            }
        }
        route_path.handlers.insert(method, handler);
    }

    pub fn register_middleware<T: Into<String>>(&mut self, name: T, handler: Middleware) {
        self.registered_middlewares.insert(name.into(), handler);
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
                if let Some(handler) = self.registered_middlewares.get(middleware) {
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

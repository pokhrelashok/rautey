use std::{collections::HashMap, path::Path, vec};

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
}
#[derive(Debug)]
pub struct RouteTree {
    path: String,
    handlers: HashMap<HTTPMethod, RouteHandler>,
    children: Vec<Box<RouteTree>>,
    middlewares: Vec<String>,
}

pub type RouteHandler =
    fn(request: Request, response: Response, route_values: HashMap<String, String>);

impl Router {
    pub fn new() -> Router {
        Router {
            middlewares: HashMap::new(),
            routes: Box::new(RouteTree {
                path: String::new(),
                handlers: HashMap::new(),
                children: vec![],
                middlewares: vec![],
            }),
        }
    }

    pub fn register(
        &mut self,
        path: &str,
        method: HTTPMethod,
        handler: RouteHandler,
        middlewares: Vec<String>,
    ) {
        let path = path.trim_matches('/');
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
        let path = request.path.trim_matches('/');
        let public_path = format!("src/public/{}", path);
        response.file(Path::new(&public_path));
    }
}

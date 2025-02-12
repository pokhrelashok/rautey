use std::{collections::HashMap, vec};

use super::{request::Request, response::Response, HTTPMethod};

pub struct Router {
    pub routes: Box<RouteTree>,
}
#[derive(Debug)]
pub struct RouteTree {
    path: String,
    handlers: HashMap<HTTPMethod, RouteHandler>,
    children: Vec<Box<RouteTree>>,
}

pub type RouteHandler = fn(request: Request, response: Response);

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Box::new(RouteTree {
                path: String::new(),
                handlers: HashMap::new(),
                children: vec![],
            }),
        }
    }

    pub fn register(&mut self, path: &str, method: HTTPMethod, func: RouteHandler) {
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
                        handlers: HashMap::new(),
                    }));

                    route_path = route_path.children.last_mut().unwrap();
                }
            }
        }
        route_path.handlers.insert(method, func);
    }

    pub fn invoke(&self, request: Request, mut response: Response) {
        let path = request.path.trim_matches('/');
        let mut route_path = &self.routes;
        let mut found = true;
        let mut was_wildcard = false;

        if path.len() > 0 {
            for dir in path.split('/') {
                let existing_index = route_path
                    .children
                    .iter()
                    .position(|child| child.path == dir || child.path == "*");

                if let Some(index) = existing_index {
                    route_path = route_path.children.get(index).unwrap();
                    if route_path.path == "*" {
                        was_wildcard = true
                    }
                } else {
                    if !was_wildcard {
                        found = false;
                    }
                    break;
                }
            }
        }

        if found && !route_path.handlers.contains_key(&request.method) {
            let has_wildcard = route_path
                .children
                .iter()
                .position(|child| child.path == "*");

            if let Some(index) = has_wildcard {
                route_path = route_path.children.get(index).unwrap()
            }
        }

        if found && (route_path.handlers.contains_key(&request.method)) {
            route_path.handlers.get(&request.method).unwrap()(request, response);
        } else {
            response.not_found();
        }
    }
}

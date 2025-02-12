use std::{collections::HashMap, fmt::format};

use super::{request::Request, response::Response, HTTPMethod};

pub struct Route {
    path: String,
    handler: fn(request: Request, response: Response),
    method: HTTPMethod,
}
pub struct Router {
    pub routes: HashMap<String, Route>,
}

pub type RouteHandler = fn(request: Request, response: Response);

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, method: HTTPMethod, func: RouteHandler) {
        let id = format!("{}-{}", method, path);
        self.routes.insert(
            id.clone(),
            Route {
                path: path.to_string(),
                handler: func,
                method,
            },
        );
    }

    pub fn has(&self, path: &str) -> bool {
        return self.routes.contains_key(path);
    }

    pub fn invoke(&self, request: Request, mut response: Response) {
        let id = format!("{}-{}", request.method, request.path);
        if self.routes.contains_key(&id) {
            ((self.routes.get(&id).unwrap()).handler)(request, response);
        } else {
            response.not_found();
        }
    }
}

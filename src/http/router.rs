use std::collections::HashMap;

use super::{response::Response, HTTPMethod};

pub struct Route {
    path: String,
    handler: fn(response: Response),
    method: HTTPMethod,
}
pub struct Router {
    pub routes: HashMap<String, Route>,
}

pub type RouteHandler = fn(response: Response);

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, method: HTTPMethod, func: fn(response: Response)) {
        self.routes.insert(
            path.to_string(),
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

    pub fn invoke(&self, path: &str, method: HTTPMethod, mut response: Response) {
        if self.routes.contains_key(path) && (self.routes.get(path).unwrap()).method == method {
            ((self.routes.get(path).unwrap()).handler)(response);
        } else {
            response.not_found();
        }
    }
}

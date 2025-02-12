use std::{collections::HashMap, net::TcpStream};

pub struct Router {
    pub routes: HashMap<String, fn(stream: TcpStream)>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, func: fn(stream: TcpStream)) {
        self.routes.insert(path.to_string(), func);
    }

    pub fn has(&self, path: &str) -> bool {
        return self.routes.contains_key(path);
    }

    pub fn invoke(&self, path: &str, stream: TcpStream) {
        if self.routes.contains_key(path) {
            self.routes.get(path).unwrap()(stream);
        }
    }
}

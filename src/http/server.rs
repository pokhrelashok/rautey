use std::{io::BufReader, net::TcpListener};

use crate::http::request::Request;

use super::{
    response::Response,
    router::{RouteHandler, Router},
    HTTPMethod,
};

pub struct Server {
    port: String,
    pub router: Router,
}
impl Server {
    pub fn new(port: &str) -> Server {
        return Server {
            port: port.to_string(),
            router: Router::new(),
        };
    }

    fn register(&mut self, path: &str, method: HTTPMethod, func: RouteHandler) {
        self.router.register(path, method, func);
    }

    pub fn get(&mut self, path: &str, func: RouteHandler) {
        self.register(path, HTTPMethod::GET, func);
    }
    pub fn post(&mut self, path: &str, func: RouteHandler) {
        self.register(path, HTTPMethod::POST, func);
    }
    pub fn delete(&mut self, path: &str, func: RouteHandler) {
        self.register(path, HTTPMethod::DELETE, func);
    }
    pub fn put(&mut self, path: &str, func: RouteHandler) {
        self.register(path, HTTPMethod::PUT, func);
    }
    pub fn patch(&mut self, path: &str, func: RouteHandler) {
        self.register(path, HTTPMethod::PATCH, func);
    }

    pub fn listen(&self) {
        println!("Server started on port {}", self.port);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .expect(format!("Port {} is already in use", self.port).as_str());
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let request = Request::parse(buf_reader);
            println!("{} request at {}", request.method, request.path);
            self.router.invoke(request, Response::new(stream))
        }
    }
}

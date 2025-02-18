use std::{error::Error, io::BufReader, net::TcpListener};

use crate::http::request::Request;

use super::{
    middleware::Middleware,
    response::Response,
    router::{RouteHandler, Router},
    HTTPMethod,
};

pub struct Server {
    port: String,
    pub router: Router,
}
impl Server {
    pub fn new<T: Into<String>>(url: T) -> Server {
        return Server {
            port: url.into(),
            router: Router::new(),
        };
    }

    fn register(
        &mut self,
        path: &str,
        method: HTTPMethod,
        func: RouteHandler,
        middlewares: Vec<String>,
    ) {
        self.router.register(path, method, func, middlewares);
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

    pub fn register_middleware<T: Into<String>>(&mut self, name: T, handler: Middleware) {
        self.router.register_middleware(name.into(), handler);
    }

    pub fn listen(&self) -> Result<(), Box<dyn Error>> {
        println!("Server started on port {}", self.port);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let request = Request::parse(buf_reader);
            println!("{} request at {}", request.method, request.path);
            self.router.invoke(request, Response::new(stream));
        }
        Ok(())
    }
}

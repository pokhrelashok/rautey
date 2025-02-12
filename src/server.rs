use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    path,
};

use crate::{http::Request, router::Router};

pub struct Server {
    port: String,
    router: Router,
}
impl Server {
    pub fn new(port: &str) -> Server {
        return Server {
            port: port.to_string(),
            router: Router::new(),
        };
    }

    pub fn register(&mut self, path: &str, func: fn(stream: TcpStream)) {
        self.router.register(path, func);
    }

    pub fn listen(&self) {
        println!("Server started on port {}", self.port);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let request = Request::parse(buf_reader);
            self.router.invoke(request.path.as_str(), stream)
        }
    }
}

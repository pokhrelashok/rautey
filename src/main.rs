mod http;
use std::path::Path;

use http::{request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new("8090");
    server.get("/", handle_home);
    server.get("/api/me", handle_me);
    server.post("/api/me", handle_me);
    server.get("/public/*", handle_public);
    server.listen();
}

fn handle_me(_: Request, mut r: Response) {
    r.json(r#""name":"Test""#);
}

fn handle_home(_: Request, mut r: Response) {
    r.file(&Path::new("src/public/index.html"));
}

fn handle_public(_: Request, mut r: Response) {
    r.text("Public path");
}

mod http;
use std::path::Path;

use http::{response::Response, server::Server};

fn main() {
    let mut server = Server::new("8090");
    server.get("/", handle_home);
    server.get("/api/me", handle_me);
    server.post("/api/me", handle_me);
    server.listen();
}

fn handle_me(mut r: Response) {
    r.json(r#""name":"Test""#);
}

fn handle_home(mut r: Response) {
    r.file(&Path::new("src/public/index.html"));
}

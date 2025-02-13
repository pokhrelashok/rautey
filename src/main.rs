mod http;
use http::{request::Request, response::Response, server::Server};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
#[derive(Serialize, Deserialize)]
struct RegisterForm {
    name: String,
    email: String,
    password: String,
}

fn main() {
    let mut server = Server::new("8090");
    server.get("/", handle_home);
    server.post("/api/register", handle_register);
    server.listen();
}

fn handle_home(_: Request, mut r: Response, _: HashMap<String, String>) {
    r.file(Path::new("src/public/index.html"));
}
fn handle_register(req: Request, mut res: Response, _: HashMap<String, String>) {
    println!("{:#?}", req);
    res.text("Success");
}

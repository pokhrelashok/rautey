mod http;
use std::{collections::HashMap, path::Path};

use http::{request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new("8090");
    server.get("/", handle_home);
    server.get("/api/me", handle_me);
    server.post("/api/me", handle_me);
    server.get("/api/users/{userId}", handle_get_user);
    server.get("/public/*", handle_public);
    server.get("/emails/{emailId}/{attachmentId}", handle_email);
    server.listen();
}

fn handle_me(_: Request, mut r: Response, _: HashMap<String, String>) {
    r.json(r#""name":"Test""#);
}

fn handle_home(_: Request, mut r: Response, _: HashMap<String, String>) {
    r.file(&Path::new("src/public/index.html"));
}

fn handle_public(_: Request, mut r: Response, _: HashMap<String, String>) {
    r.text("Public path");
}

fn handle_get_user(_: Request, mut res: Response, pams: HashMap<String, String>) {
    let user_id: String = pams.get("userId").unwrap_or(&String::from("")).to_owned();
    res.text(format!("You asked for userID {}", user_id).as_str());
}
fn handle_email(_: Request, mut res: Response, pams: HashMap<String, String>) {
    res.text(format!("Passed details {:#?}", pams).as_str());
}

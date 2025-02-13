mod http;
use http::{request::Request, response::Response, server::Server};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
fn default_gender() -> String {
    "Male".to_string()
}
#[derive(Serialize, Deserialize, Debug)]

struct RegisterForm {
    name: String,
    email: String,
    age: String,
    #[serde(default = "default_gender")]
    gender: String,
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
    let body = req.parse_body::<RegisterForm>();
    if let Ok(body) = body {
        println!("{:#?}", body);
    }
    res.text("Success");
}

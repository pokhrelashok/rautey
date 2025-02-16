mod http;
use http::{cookie::Cookie, request::Request, response::Response, server::Server};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, thread, time::Duration};
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
    server.post("/api/users/{id}", get_user_details);
    server.post("/api/register", handle_register);
    server.listen();
}

fn handle_home(_: Request, mut r: Response, _: HashMap<String, String>) {
    r.file(Path::new("src/public/index.html"));
}

fn handle_register(req: Request, mut res: Response, _: HashMap<String, String>) {
    let body = req.parse_body::<RegisterForm>();
    if let Ok(body) = body {
        if body.files.contains_key("cv") {
            let cv_file = body.files.get("cv").unwrap();
            cv_file
                .uploader()
                .with_filename("test_filename")
                .with_path("src/storage/test_uploads")
                .upload()
                .unwrap();
        }
    }
    res.text("Success");
}

fn get_user_details(_: Request, mut res: Response, params: HashMap<String, String>) {
    res.with_cookie(
        Cookie::new(String::from("Hello"), String::from("world"))
            .http_only()
            .secure(),
    )
    .text(format!(
        "You were requesting user_id {}",
        params.get("id").unwrap()
    ));
}

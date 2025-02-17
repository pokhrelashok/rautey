mod http;
use dotenvy::var;
use http::{cookie::Cookie, request::Request, response::Response, server::Server};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
#[derive(Serialize, Deserialize, Debug)]

struct RegisterForm {
    name: String,
    email: String,
    age: String,
}

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server.register_middleware("admin-only", admin_only_middleware);

    server.get("/", handle_home, None);
    server.get("/api/users/{id}", get_user_details, None);
    server.post("/api/register", handle_register, None);
    server.get(
        "/admin",
        handle_admin_route,
        Some(vec!["admin-only".to_string()]),
    );
    server.listen();
}

fn handle_home(req: Request, mut r: Response, _: HashMap<String, String>) {
    r.file(Path::new("src/public/index.html"));
}

fn handle_register(req: Request, mut res: Response, _: HashMap<String, String>) {
    let body = req.parse_body::<RegisterForm>();
    if let Ok(body) = body {
        if body.files.contains_key("cv") {
            let cv_file = body.files.get("cv").unwrap();
            cv_file.uploader().upload().unwrap();
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

fn handle_admin_route(req: Request, mut res: Response, _: HashMap<String, String>) {
    res.text("Welcome to admint dashboard");
}
fn admin_only_middleware(req: &Request, res: &mut Response, _: &HashMap<String, String>) {
    res.redirect("/api/users/112");
}

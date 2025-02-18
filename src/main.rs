mod http;
use dotenvy::var;
use http::{cookie::Cookie, request::Request, response::Response, router::Router, server::Server};
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
    server
        .router
        .register_middleware("middleware-1", middleware_1);
    server
        .router
        .register_middleware("middleware-2", middleware_2);
    server.router.group(
        "/api",
        Some(vec![String::from("middleware-1")]),
        |router: &mut Router| {
            router.get("/", handle_home, Some(vec![String::from("middleware-2")]));
            router.get("/users/{id}", get_user_details, None);
            router.post("/register", handle_register, None);
            router.get("/admin", handle_admin_route, None);
        },
    );
    server.listen().expect("Could not bind port");
}

fn handle_home(req: Request, mut r: Response, _: HashMap<String, String>) {
    r.file(Path::new("public/index.html"));
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
    res.text(format!(
        "You were requesting user_id {}",
        params.get("id").unwrap()
    ));
}

fn handle_admin_route(req: Request, mut res: Response, _: HashMap<String, String>) {
    res.text("Welcome to admint dashboard");
}
fn middleware_1(req: &Request, res: &mut Response, _: &HashMap<String, String>) {
    res.with_cookie(Cookie::new("1", "1"));
}

fn middleware_2(req: &Request, res: &mut Response, _: &HashMap<String, String>) {
    res.with_cookie(Cookie::new("2", "2"));
}

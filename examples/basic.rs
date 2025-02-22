use dotenvy::var;
use rautey::{
    cookie::Cookie, request::Request, response::Response, router::Router, server::Server,
    session::SessionStore,
};
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
    server
        .router
        .register_middleware("middleware-3", middleware_3);
    server.router.group(
        "/api",
        Some(&[String::from("session")]),
        |router: &mut Router| {
            router.get(
                "/",
                handle_api_home,
                Some(vec![String::from("middleware-2")]),
            );
            router.group(
                "/v2",
                Some(&["middleware-3".to_string()]),
                |router: &mut Router| {
                    router.get("/", handle_home, Some(vec![String::from("middleware-2")]));
                    router.get("/users/{id}", get_user_details, None);
                    router.post("/register", handle_register, None);
                    router.get("/admin", handle_admin_route, None);
                },
            );
        },
    );
    server.listen().expect("Could not bind port");
}

fn handle_home(req: Request, mut r: Response, _: HashMap<String, String>) {
    r.file(Path::new("public/index.html"));
}
fn handle_api_home(req: Request, mut r: Response, _: HashMap<String, String>) {
    println!(
        "The user_id is {}",
        req.session.get::<String>("user_id").unwrap()
    );
    r.text("Api home");
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

fn get_user_details(mut req: Request, mut res: Response, params: HashMap<String, String>) {
    req.session
        .set("user_id", params.get("id").unwrap(), &mut res);
    res.text(format!(
        "You were requesting user_id {}",
        params.get("id").unwrap()
    ));
}

fn handle_admin_route(req: Request, mut res: Response, _: HashMap<String, String>) {
    res.text("Welcome to admint dashboard");
}
fn middleware_1(req: &Request, res: &mut Response, _: &HashMap<String, String>) {
    println!("1 called");
    res.with_cookie(Cookie::new("1", "1"));
}

fn middleware_2(req: &Request, res: &mut Response, _: &HashMap<String, String>) {
    println!("2 called");
    res.with_cookie(Cookie::new("2", "2"));
}

fn middleware_3(req: &Request, res: &mut Response, _: &HashMap<String, String>) {
    println!("3 called");
    res.with_cookie(Cookie::new("3", "3"));
}

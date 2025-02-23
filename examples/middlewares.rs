use dotenvy::var;
use rautey::{request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server
        .router
        .register_middleware("is-logged-in", |req: &Request, res: &mut Response| {
            if !req.cookies.contains_key("auth_token") {
                res.with_status(rautey::HTTPStatus::UNAUTHORIZED)
                    .text("Permission denied");
            }
        });

    server
        .router
        .get("/users/{id}", get_user_details)
        .with_middlewares(["is-logged-in"]);
    server.listen().expect("Could not bind port");
}

fn get_user_details(req: Request, mut r: Response) {
    r.text(format!(
        "User id is {}",
        req.route_params.get("id").unwrap()
    ));
}

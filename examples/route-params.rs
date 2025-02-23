use dotenvy::var;
use rautey::{request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server
        .router
        .get("/users/{id}", |req: Request, mut r: Response| {
            r.text(format!(
                "User id is {}",
                req.route_params.get("id").unwrap()
            ));
        });
    server.listen().expect("Could not bind port");
}

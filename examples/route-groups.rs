use dotenvy::var;
use rautey::{request::Request, response::Response, router::Router, server::Server};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server.router.group("/api", |router: &mut Router| {
        router.get("/", handle_api_home);
        router.group("/v2", |router: &mut Router| {
            router.get("/", handle_v2_home);
        });
    });
    server.listen().expect("Could not bind port");
}

fn handle_api_home(_: Request, mut r: Response) {
    r.text("This is api home");
}

fn handle_v2_home(_: Request, mut res: Response) {
    res.text("This is v2 home");
}

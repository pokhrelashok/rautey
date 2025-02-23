use dotenvy::var;
use rautey::{request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server.router.get("/", |_: Request, mut res: Response| {
        res.text("Hello World");
    });
    server.listen().expect("Could not bind port");
}

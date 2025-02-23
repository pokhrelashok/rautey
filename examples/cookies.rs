use dotenvy::var;
use rautey::{cookie::Cookie, request::Request, response::Response, server::Server};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server.router.get("/", |req: Request, mut r: Response| {
        println!("{:?}", req.cookies); //cookies in request

        // add cookies to response
        r.with_cookie(Cookie::new("visited", "yes"))
            .text("Cookies example");
    });
    server.listen().expect("Could not bind port");
}

use dotenvy::var;
use rautey::{request::Request, response::Response, server::Server, session::SessionStore};

fn main() {
    let mut server = Server::new(var("APP_PORT").unwrap());
    server
        .router
        .get("/", |mut req: Request, mut res: Response| {
            // access request sesion
            println!(
                "{:?}",
                req.session.get::<String>("session_id").unwrap_or_default()
            );

            req.session.set("session_id", "1234", &mut res);
            res.text("Session example");
        });
    server.listen().expect("Could not bind port");
}

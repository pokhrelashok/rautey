mod http;
use http::{response::Response, server::Server};

fn main() {
    let mut server = Server::new("8090");
    server.get("/", handle_me);
    server.listen();
}

fn handle_me(mut r: Response) {
    r.json(r#"{"name":"Ashok"}"#);
}

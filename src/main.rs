mod http;
mod pool;
mod router;
mod server;

use std::{io::Write, net::TcpStream};

use server::Server;

fn main() {
    let mut server = Server::new("8090");
    server.register("/api/me", handle_me);
    server.listen();
}

fn handle_me(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

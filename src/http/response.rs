use std::{io::Write, net::TcpStream};

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        return Response { stream };
    }

    pub fn not_found(&mut self) {
        self.stream
            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .unwrap();
    }

    pub fn json(&mut self, json: &str) {
        self.stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{json}")
                    .as_bytes(),
            )
            .unwrap();
    }
}

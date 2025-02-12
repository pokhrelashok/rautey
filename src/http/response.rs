use std::{
    fs::{self},
    io::Write,
    net::TcpStream,
    path::Path,
};

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
        let len = json.len();
        self.stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {len}\r\n\r\n{json}")
                    .as_bytes(),
            )
            .unwrap();
    }

    pub fn file(&mut self, path: &Path) {
        let file_content = fs::read_to_string(path).expect("Cannot open the file");
        let len = file_content.len();
        let extension = path.extension().unwrap_or_default().to_string_lossy();
        self.stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: text/{extension}\r\nContent-Length: {len}\r\n\r\n{file_content}")
                    .as_bytes(),
            )
            .unwrap();
    }
}

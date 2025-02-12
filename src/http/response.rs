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

    pub fn success(&mut self, content: &str, ext: &str) {
        let len = content.len();
        self.stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: {ext}\r\nContent-Length: {len}\r\n\r\n{content}")
                    .as_bytes(),
            )
            .unwrap();
    }

    pub fn json(&mut self, json: &str) {
        self.success(json, "application/json");
    }

    pub fn text(&mut self, text: &str) {
        self.success(text, "text/plain");
    }

    pub fn file(&mut self, path: &Path) {
        let file_content = fs::read_to_string(path).expect("Cannot open the file");
        let extension = path.extension().unwrap_or_default().to_string_lossy();
        self.success(&file_content, &format!("text/{extension}"));
    }
}

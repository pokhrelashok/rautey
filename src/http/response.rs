use std::{
    fs::{self, File},
    io::{Read, Write},
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
        if let Ok(mut file) = File::open(path) {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .expect("Failed to read file");
            let content_type = match path.extension().and_then(|ext| ext.to_str()) {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("json") => "application/json",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("pdf") => "application/pdf",
                _ => "application/octet-stream",
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\n\
            Content-Length: {}\r\n\
            Content-Type: {}\r\n\
            Connection: close\r\n\
            \r\n",
                contents.len(),
                content_type,
            );

            self.stream
                .write_all(response.as_bytes())
                .expect("Failed to send response");
            self.stream
                .write_all(&contents)
                .expect("Failed to send file");
        } else {
            self.not_found();
        }
    }
}

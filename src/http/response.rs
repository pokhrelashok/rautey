use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

pub struct Response {
    stream: TcpStream,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        return Response {
            stream,
            headers: HashMap::new(),
        };
    }
    pub fn add_header<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) {
        self.headers.insert(key.into(), value.into());
    }
    pub fn not_found(&mut self) {
        self.stream
            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .unwrap();
    }

    pub fn success<T: AsRef<str>, S: AsRef<str>>(&mut self, content: T, ext: S) {
        let content = content.as_ref();
        let ext = ext.as_ref();
        let len = content.len();
        let mut res = format!("HTTP/1.1 200 OK\r\nContent-Type: {ext}\r\nContent-Length: {len}");

        for (key, val) in &mut self.headers {
            res.push_str(&format!("\r\n{key}: {val}"));
        }
        res.push_str(&format!("\r\n\r\n{content}"));
        self.stream.write_all(res.as_bytes()).unwrap();
    }

    pub fn json<T: AsRef<str>>(&mut self, json: T) {
        self.success(json, "application/json");
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) {
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

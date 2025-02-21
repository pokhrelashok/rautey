use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    path::Path,
};

use super::{cookie::Cookie, HTTPStatus};
#[derive(Debug)]
pub struct Response {
    stream: TcpStream,
    headers: HashMap<String, String>,
    cookies: Vec<Cookie>,
    status: HTTPStatus,
}

impl Response {
    pub fn new(stream: TcpStream) -> Response {
        return Response {
            stream,
            status: HTTPStatus::SUCCESS,
            headers: HashMap::new(),
            cookies: vec![],
        };
    }

    pub fn with_status(&mut self, status: HTTPStatus) -> &mut Self {
        self.status = status;
        self
    }

    pub fn with_headers<T: Into<String>, S: Into<String>>(
        &mut self,
        headers: HashMap<T, S>,
    ) -> &mut Self {
        for (k, v) in headers {
            self.headers.insert(k.into(), v.into());
        }
        self
    }

    pub fn with_header<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_cookie(&mut self, cookie: Cookie) -> &mut Self {
        self.cookies.push(cookie);
        self
    }

    pub fn with_cookies(&mut self, mut cookies: Vec<Cookie>) -> &mut Self {
        self.cookies.append(&mut cookies);
        self
    }

    pub fn not_found(&mut self) {
        self.status = HTTPStatus::NOT_FOUND;
        self.respond(b"", "text/plain");
    }

    fn respond<S: AsRef<str>>(&mut self, content: &[u8], content_type: S) {
        let status_line = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.status_code(),
            self.status.status_text()
        );
        let mut headers = String::new();
        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }
        let mut cookies = String::new();
        for cookie in &self.cookies {
            cookies.push_str(&format!("Set-Cookie: {}\r\n", cookie.to_string()));
        }
        let response = format!(
            "{}{}{}Connection: close\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
            status_line,
            headers,
            cookies,
            content.len(),
            content_type.as_ref()
        );
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.write_all(content).unwrap();
    }

    pub fn json<T: AsRef<str>>(&mut self, json: T) {
        self.respond(json.as_ref().as_bytes(), "application/json");
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) {
        self.respond(text.as_ref().as_bytes(), "text/plain");
    }

    pub fn file(&mut self, path: &Path) {
        if path.is_file() && path.exists() {
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
                self.respond(&contents, content_type);
            } else {
                self.not_found();
            }
        } else {
            self.not_found();
        }
    }

    pub fn redirect<T: AsRef<str>>(&mut self, to: T) {
        self.with_status(HTTPStatus::REDIRECT)
            .with_header("Location", to.as_ref())
            .text("");
    }
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub uri: String,
    pub path: String,
    pub body: String,
    pub query: HashMap<String, String>,
}

impl Request {
    pub fn parse(mut buf_reader: BufReader<&TcpStream>) -> Request {
        let mut method = HTTPMethod::GET;
        let mut uri = String::new();
        let mut length = 0 as u16;
        let mut path = String::new();
        let mut query = HashMap::new();

        let headers: Vec<String> = buf_reader
            .by_ref()
            .lines()
            .map(|f| f.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut lines = headers.iter();
        let first_line = lines.next().unwrap();
        let mut words = first_line.split_whitespace();

        match words.next().unwrap() {
            "POST" => method = HTTPMethod::POST,
            "PUT" => method = HTTPMethod::PUT,
            "DELETE" => method = HTTPMethod::DELETE,
            _ => {}
        };

        uri = words.next().unwrap().to_string();

        for line in lines {
            if line.starts_with("Content-Length") {
                length = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .parse::<u16>()
                    .unwrap();
            }
        }

        let mut body = vec![0; length as usize]; //New Vector with size of Content
        buf_reader.read_exact(&mut body).unwrap(); //Get the Body Content.
        let body = String::from_utf8_lossy(&body).to_string();

        let mut path_params = uri.split("?");
        path = path_params.nth(0).unwrap_or_default().to_owned();
        let query_params = path_params.nth(0).unwrap_or_default().to_owned();
        let key_vals: Vec<&str> = query_params.split("&").collect();
        for pair in key_vals {
            let mut spl = pair.split("=");
            let key = spl.nth(0).unwrap_or_default().to_owned();
            let val = spl.nth(0).unwrap_or_default().to_owned();
            query.insert(key, val);
        }

        Request {
            method,
            uri,
            body,
            query,
            path,
        }
    }
}

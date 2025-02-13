use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use serde::Deserialize;
use urlencoding::decode;

use super::HTTPMethod;

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub uri: String,
    pub path: String,
    pub body: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn parse(mut buf_reader: BufReader<&TcpStream>) -> Request {
        let mut method = HTTPMethod::GET;
        let mut uri = String::new();
        let mut length = 0 as u16;
        let mut path = String::new();
        let mut query = HashMap::new();
        let mut headers = HashMap::new();

        let all_headers: Vec<String> = buf_reader
            .by_ref()
            .lines()
            .map(|f| f.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut lines = all_headers.iter();
        let first_line = lines.next().unwrap();
        let mut words = first_line.split_whitespace();

        match words.next().unwrap() {
            "POST" => method = HTTPMethod::POST,
            "PUT" => method = HTTPMethod::PUT,
            "DELETE" => method = HTTPMethod::DELETE,
            "PATCH" => method = HTTPMethod::PATCH,
            _ => method = HTTPMethod::GET,
        };

        uri = words.next().unwrap().to_string();

        // Extract request headers
        for line in lines {
            let mut header_value = line.trim().split(":");
            let h = header_value.next().unwrap_or_default().trim();
            let v = header_value.next().unwrap_or_default().trim();
            headers.insert(h.to_string(), v.to_string());
        }

        // Extract request body
        length = headers
            .get("Content-Length")
            .unwrap()
            .trim()
            .parse::<u16>()
            .unwrap();

        let mut body = vec![0; length as usize];
        buf_reader.read_exact(&mut body).unwrap();
        let body = String::from_utf8_lossy(&body).to_string();

        // Extract Query Params from request
        let mut path_params = uri.split("?");
        path = path_params.nth(0).unwrap_or_default().to_owned();
        let query_params = path_params.nth(0);
        if let Some(qp) = query_params {
            let key_vals: Vec<&str> = qp.split("&").collect();
            for pair in key_vals {
                let mut spl = pair.split("=");
                let key = spl.nth(0).unwrap_or_default().to_owned();
                let val = spl.nth(0).unwrap_or_default().to_owned();
                query.insert(key, val);
            }
        }

        Request {
            method,
            uri,
            body,
            query,
            path,
            headers,
        }
    }

    pub fn parse_body<T>(&self) -> Option<Result<T, serde_json::Error>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let content_type = self
            .headers
            .get("Content-Type")
            .unwrap_or(&String::from(""))
            .to_owned();
        println!("{content_type} {}", self.body);
        if content_type == "application/json" {
            return match serde_json::from_str(&self.body) {
                Ok(parsed) => Some(Ok(parsed)),
                Err(e) => Some(Err(e)),
            };
        } else if content_type == "application/x-www-form-urlencoded" {
            let decoded = urlencoded_to_json(&self.body);

            return match serde_json::to_value(decoded) {
                Ok(json) => match T::deserialize(json) {
                    Ok(v) => Some(Ok(v)),
                    Err(e) => Some(Err(e)),
                },
                Err(e) => Some(Err(e)),
            };
        }
        return None;
    }
}

fn urlencoded_to_json(content: &str) -> HashMap<String, String> {
    let content = decode(content).expect("Invalid data");
    let key_val_pairs = content.split("&");
    let mut result = HashMap::new();
    for key_val_pair in key_val_pairs {
        let mut key_val = key_val_pair.split("=");
        let k = key_val.nth(0).unwrap_or_default().to_string();
        let v = key_val.nth(0).unwrap_or_default().to_string();
        if !k.is_empty() && !v.is_empty() {
            result.insert(k, v);
        }
    }

    return result;
}

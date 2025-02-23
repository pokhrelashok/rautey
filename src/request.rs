use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use dotenvy::var;
use serde::Deserialize;
use serde_json::Error;

use crate::session::{NoSession, SessionBackend, SessionStore};

use super::{
    file::UploadedFile,
    parsers::{parse_multipart_form_data, parse_url_encoded},
    session::FileSession,
    utils::uuid,
    HTTPMethod,
};
#[derive(Debug)]
pub struct RequestBody<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub data: Option<T>,
    pub files: HashMap<String, UploadedFile>,
}

#[derive(Debug)]
pub struct Request {
    pub method: HTTPMethod,
    pub uri: String,
    pub path: String,
    pub body: Vec<u8>,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub route_params: HashMap<String, String>,
    pub session: SessionBackend,
    pub cookies: HashMap<String, String>,
}

impl Request {
    pub fn parse(mut buf_reader: BufReader<&TcpStream>) -> Request {
        let mut method = HTTPMethod::GET;
        let mut uri = String::new();
        let mut length = 0 as u32;
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
            .unwrap_or(&"0".to_string())
            .trim()
            .parse::<u32>()
            .unwrap();

        let mut body = vec![0; length as usize];
        buf_reader.read_exact(&mut body).unwrap();

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

        // extract cookies
        let cookies = if headers.contains_key("Cookie") {
            parse_cookies(headers.get("Cookie").unwrap())
        } else {
            HashMap::new()
        };

        Request {
            method,
            uri,
            body,
            query,
            path,
            headers,
            cookies,
            route_params: HashMap::new(),
            session: SessionBackend::NoSession(NoSession {}),
        }
    }

    pub fn parse_body<T>(&self) -> Result<RequestBody<T>, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let content_type = self
            .headers
            .get("Content-Type")
            .unwrap_or(&String::from(""))
            .to_owned();

        if content_type == "application/json" {
            return match serde_json::from_str(&String::from_utf8_lossy(&self.body)) {
                Ok(parsed) => Ok(RequestBody {
                    data: parsed,
                    files: HashMap::new(),
                }),
                Err(e) => Err(e),
            };
        } else if content_type == "application/x-www-form-urlencoded" {
            let decoded = parse_url_encoded(&String::from_utf8_lossy(&self.body));

            return match serde_json::to_value(decoded) {
                Ok(json) => match T::deserialize(json) {
                    Ok(v) => Ok(RequestBody {
                        data: Some(v),
                        files: HashMap::new(),
                    }),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            };
        } else if content_type.contains("multipart/form-data") {
            let boundary = content_type
                .split("; ")
                .find(|s| s.starts_with("boundary="))
                .map(|s| s.trim_start_matches("boundary=").to_string())
                .unwrap();
            let (decoded, files) = parse_multipart_form_data(&self.body, &boundary);
            return match serde_json::to_value(decoded) {
                Ok(json) => match T::deserialize(json) {
                    Ok(v) => Ok(RequestBody {
                        data: Some(v),
                        files: files,
                    }),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            };
        } else {
            return Ok(RequestBody {
                data: None,
                files: HashMap::new(),
            });
        }
    }
}
fn parse_cookies<T: AsRef<str>>(cookie: T) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    for cookie_line in cookie.as_ref().split(";") {
        let (k, v) = cookie_line.split_once("=").unwrap_or_default();
        if k.len() > 0 {
            cookies.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    cookies
}

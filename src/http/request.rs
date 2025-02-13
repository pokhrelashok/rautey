use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use serde::Deserialize;
use serde_json::Error;
use urlencoding::decode;

use super::HTTPMethod;
#[derive(Debug)]
pub struct Body<T>
where
    T: for<'de> Deserialize<'de>,
{
    pub data: Option<T>,
    pub files: HashMap<String, FileData>,
}

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
            .unwrap()
            .trim()
            .parse::<u32>()
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

    pub fn parse_body<T>(&self) -> Result<Body<T>, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let content_type = self
            .headers
            .get("Content-Type")
            .unwrap_or(&String::from(""))
            .to_owned();
        if content_type == "application/json" {
            return match serde_json::from_str(&self.body) {
                Ok(parsed) => Ok(Body {
                    data: parsed,
                    files: HashMap::new(),
                }),
                Err(e) => Err(e),
            };
        } else if content_type == "application/x-www-form-urlencoded" {
            let decoded = decode_url_encoded(&self.body);

            return match serde_json::to_value(decoded) {
                Ok(json) => match T::deserialize(json) {
                    Ok(v) => Ok(Body {
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
                    Ok(v) => Ok(Body {
                        data: Some(v),
                        files: files,
                    }),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            };
        } else {
            return Ok(Body {
                data: None,
                files: HashMap::new(),
            });
        }
    }
}

fn decode_url_encoded(content: &str) -> HashMap<String, String> {
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

#[derive(Debug)]
pub struct FileData {
    pub filename: String,
    pub content_type: Option<String>,
    pub content: Vec<u8>,
    pub extension: String,
    pub size: u64,
}

fn parse_multipart_form_data(
    form_data: &str,
    boundary: &str,
) -> (HashMap<String, String>, HashMap<String, FileData>) {
    let mut fields = HashMap::new();
    let mut files = HashMap::new();
    let boundary_marker = format!("--{}", boundary);
    let end_marker = format!("--{}--", boundary);

    for part in form_data.split(&boundary_marker).skip(1) {
        let part = part.trim();
        if part == end_marker {
            break;
        }

        if let Some((headers, body)) = part.split_once("\r\n\r\n") {
            let body = body.trim_end_matches("\r\n").as_bytes().to_vec(); // Handle as bytes for files

            let mut field_name = None;
            let mut filename = None;
            let mut content_type = None;

            for header in headers.lines() {
                if header.starts_with("Content-Disposition: form-data;") {
                    if let Some(name_part) = header.split("name=").nth(1) {
                        field_name = Some(name_part.trim_matches(&['"', '\''][..]).to_string());
                    }
                    if let Some(filename_part) = header.split("filename=").nth(1) {
                        filename = Some(filename_part.trim_matches(&['"', '\''][..]).to_string());
                    }
                } else if header.starts_with("Content-Type:") {
                    content_type = Some(header.split_once(": ").unwrap().1.to_string());
                }
            }

            if let Some(name) = field_name {
                if let Some(file_name) = filename {
                    files.insert(
                        name,
                        FileData {
                            extension: file_name.split(".").last().unwrap_or_default().to_owned(),
                            filename: file_name,
                            content_type,
                            size: body.len() as u64,
                            content: body,
                        },
                    );
                } else {
                    fields.insert(name, String::from_utf8_lossy(&body).to_string());
                }
            }
        }
    }

    (fields, files)
}

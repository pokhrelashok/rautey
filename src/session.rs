use std::{
    collections::HashMap,
    error::Error,
    fmt::Debug,
    fs::{write, OpenOptions},
    io::{Read, Seek},
};

use serde_json::{from_slice, from_str, to_value, Value};
use urlencoding::{decode, encode};

use crate::{cookie::Cookie, request::Request, response::Response};

#[derive(Debug)]
pub enum SessionBackend {
    File(FileSession),
    NoSession(NoSession),
    Cookie(CookieSession),
}

#[derive(Debug)]
pub struct FileSession {
    pub id: String,
    data: HashMap<String, Value>,
}

pub trait SessionStore: Debug {
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T>;
    fn set<T: serde::Serialize>(&mut self, key: &str, value: T, response: &mut Response);
    fn init(&mut self, request: &Request) -> Result<(), Box<dyn Error>>;
    fn id(&self) -> &str;
}

impl SessionStore for SessionBackend {
    fn init(&mut self, request: &Request) -> Result<(), Box<dyn Error>> {
        match self {
            SessionBackend::File(s) => s.init(),
            SessionBackend::NoSession(_) => Ok(()),
            SessionBackend::Cookie(s) => s.init(request),
        }
    }
    fn set<T: serde::Serialize>(&mut self, key: &str, val: T, response: &mut Response) {
        match self {
            SessionBackend::File(s) => s.set(key, val),
            SessionBackend::NoSession(_) => {}
            SessionBackend::Cookie(s) => s.set(key, val, response),
        }
    }
    fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self {
            SessionBackend::File(s) => s.get(key),
            SessionBackend::NoSession(_) => None,
            SessionBackend::Cookie(s) => s.get(key),
        }
    }

    fn id(&self) -> &str {
        match self {
            SessionBackend::File(s) => &s.id,
            SessionBackend::NoSession(_) => "",
            SessionBackend::Cookie(s) => &s.id,
        }
    }
}

impl FileSession {
    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("sessions/{}", self.id))?;

        let mut data = vec![];
        file.seek(std::io::SeekFrom::Start(0))?;
        file.read_to_end(&mut data)?;
        let json_value: HashMap<String, Value> = if data.is_empty() {
            HashMap::new()
        } else {
            from_slice(&data)?
        };
        self.data = json_value;
        Ok(())
    }

    fn save_to_file<T: AsRef<str>>(&self, path: T) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string_pretty(&self.data)?;
        write(path.as_ref(), json_str)?;
        Ok(())
    }
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        return FileSession {
            id: id.as_ref().to_string(),
            data: HashMap::new(),
        };
    }
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        serde_json::from_value(self.data.get(key)?.clone()).ok()
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) {
        if let Ok(json_value) = to_value(value) {
            self.data.insert(key.to_string(), json_value);
            self.save_to_file(format!("sessions/{}", self.id)).unwrap();
        }
    }
}
#[derive(Debug)]

pub struct NoSession {}

impl NoSession {
    pub fn new() -> Self {
        return NoSession {};
    }
}

#[derive(Debug)]
pub struct CookieSession {
    pub id: String,
    data: HashMap<String, Value>,
}

impl CookieSession {
    pub fn init(&mut self, request: &Request) -> Result<(), Box<dyn Error>> {
        let session_in_cookie: String = request
            .cookies
            .get("session")
            .map_or(String::new(), |f| f.to_owned());
        let session_val: HashMap<String, Value> = if session_in_cookie.is_empty() {
            HashMap::new()
        } else {
            let decoded = decode(&session_in_cookie).unwrap_or_default();
            from_str(&decoded).unwrap()
        };
        self.data = session_val;
        println!("{:?}", self.data);
        Ok(())
    }

    pub fn new<T: AsRef<str>>(id: T) -> Self {
        return CookieSession {
            id: id.as_ref().to_string(),
            data: HashMap::new(),
        };
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        serde_json::from_value(self.data.get(key)?.clone()).ok()
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T, res: &mut Response) {
        if let Ok(json_value) = to_value(value) {
            self.data.insert(key.to_string(), json_value);
            let mut json_str = serde_json::to_string_pretty(&self.data).unwrap();
            json_str = json_str.replace(" ", "").replace("\n", "");
            res.with_cookie(Cookie::new("session", encode(&json_str)));
        };
    }
}

use std::{
    collections::HashMap,
    error::Error,
    fs::{write, OpenOptions},
    io::{Read, Seek},
};

use serde_json::{from_slice, to_value, Value};

#[derive(Debug)]
pub struct Session {
    pub id: String,
    data: HashMap<String, Value>,
}

impl Session {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        return Session {
            id: id.as_ref().to_string(),
            data: HashMap::new(),
        };
    }
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

    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        serde_json::from_value(self.data.get(key)?.clone()).ok()
    }

    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) {
        if let Ok(json_value) = to_value(value) {
            self.data.insert(key.to_string(), json_value);
            self.save_to_file(format!("sessions/{}", self.id)).unwrap();
        }
    }

    fn save_to_file<T: AsRef<str>>(&self, path: T) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string_pretty(&self.data)?;
        write(path.as_ref(), json_str)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Cookie {
    pub key: String,
    pub value: String,
    pub expires: Option<String>,
    pub secure: bool,
    pub max_age: Option<u64>,
    pub http_only: bool,
    pub path: String,
}

impl Cookie {
    pub fn new<T: Into<String>, S: Into<String>>(key: T, value: S) -> Self {
        Cookie {
            key: key.into(),
            value: value.into(),
            expires: None,
            secure: false,
            max_age: None,
            http_only: false,
            path: "/".to_string(),
        }
    }

    pub fn with_expires(mut self, expires: String) -> Self {
        self.expires = Some(expires);
        self
    }

    pub fn secure(mut self) -> Self {
        self.secure = true;
        self
    }

    pub fn with_max_age(mut self, max_age: u64) -> Self {
        self.max_age = Some(max_age);
        self
    }

    pub fn http_only(mut self) -> Self {
        self.http_only = true;
        self
    }
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }
}

impl ToString for Cookie {
    fn to_string(&self) -> String {
        let mut cookie_string = format!("{}={}", self.key, self.value);

        if let Some(ref expires) = self.expires {
            cookie_string.push_str(&format!("; Expires={}", expires));
        }

        if self.secure {
            cookie_string.push_str("; Secure");
        }

        if let Some(max_age) = self.max_age {
            cookie_string.push_str(&format!("; Max-Age={}", max_age));
        }

        if self.http_only {
            cookie_string.push_str("; HttpOnly");
        }

        cookie_string.push_str(&format!("; Path={}", self.path));
        cookie_string
    }
}

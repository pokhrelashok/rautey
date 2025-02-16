use std::fmt::{Debug, Display};

#[derive(Debug, Eq, PartialEq, Hash)]

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}
#[derive(Debug, Eq, PartialEq, Hash)]

pub enum HTTPStatus {
    SUCCESS,
    NOT_FOUND,
    SERVER_ERROR,
    BAD_REQUEST,
    UNAUTHORIZED,
    FORBIDDEN,
    INTERNAL_SERVER_ERROR,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
}

impl HTTPStatus {
    pub fn status_text(&self) -> &str {
        match self {
            HTTPStatus::SUCCESS => "OK",
            HTTPStatus::NOT_FOUND => "Not Found",
            HTTPStatus::SERVER_ERROR => "Internal Server Error",
            HTTPStatus::BAD_REQUEST => "Bad Request",
            HTTPStatus::UNAUTHORIZED => "Unauthorized",
            HTTPStatus::FORBIDDEN => "Forbidden",
            HTTPStatus::INTERNAL_SERVER_ERROR => "Internal Server Error",
            HTTPStatus::BAD_GATEWAY => "Bad Gateway",
            HTTPStatus::SERVICE_UNAVAILABLE => "Service Unavailable",
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            HTTPStatus::SUCCESS => 200,
            HTTPStatus::NOT_FOUND => 404,
            HTTPStatus::SERVER_ERROR => 500,
            HTTPStatus::BAD_REQUEST => 400,
            HTTPStatus::UNAUTHORIZED => 401,
            HTTPStatus::FORBIDDEN => 403,
            HTTPStatus::INTERNAL_SERVER_ERROR => 500,
            HTTPStatus::BAD_GATEWAY => 502,
            HTTPStatus::SERVICE_UNAVAILABLE => 503,
        }
    }
}

impl Display for HTTPStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub mod cookie;
pub mod file;
mod parsers;
pub mod request;
pub mod response;
pub mod router;
pub mod server;

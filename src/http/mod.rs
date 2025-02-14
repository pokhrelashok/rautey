use std::fmt::{Debug, Display};

#[derive(Debug, Eq, PartialEq, Hash)]

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}
impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub mod file;
mod parsers;
pub mod request;
pub mod response;
pub mod router;
pub mod server;

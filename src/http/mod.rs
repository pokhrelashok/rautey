#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}
impl PartialEq for HTTPMethod {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub mod request;
pub mod response;
pub mod router;
pub mod server;

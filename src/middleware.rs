use std::collections::HashMap;

use crate::cookie::Cookie;

use super::{request::Request, response::Response};

pub type Middleware =
    fn(request: &Request, response: &mut Response, route_values: &HashMap<String, String>);

pub fn session_handler(request: &Request, response: &mut Response, _: &HashMap<String, String>) {
    response.with_cookie(Cookie::new("session_id", &request.session.id));
}

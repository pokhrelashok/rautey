use std::collections::HashMap;

use super::{request::Request, response::Response};

pub type Middleware =
    fn(request: &Request, response: &mut Response, route_values: &HashMap<String, String>);

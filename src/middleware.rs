use crate::{cookie::Cookie, session::SessionStore};

use super::{request::Request, response::Response};

pub type Middleware = fn(request: &Request, response: &mut Response);

pub fn session_handler(request: &Request, response: &mut Response) {
    let sess_id = request.session.id();
    response.with_cookie(Cookie::new("session_id", sess_id));
}

use std::{env::var, error::Error, io::BufReader, net::TcpListener, ops::DerefMut};

use crate::{
    request::Request,
    session::{CookieSession, FileSession, NoSession, SessionBackend, SessionStore},
    utils::uuid,
};

use super::{
    response::Response,
    router::{RouteHandler, Router},
    HTTPMethod,
};

pub struct Server {
    port: String,
    pub router: Router,
}
impl Server {
    pub fn new<T: Into<String>>(url: T) -> Server {
        return Server {
            port: url.into(),
            router: Router::new(),
        };
    }

    fn register(
        &mut self,
        path: &str,
        method: HTTPMethod,
        func: RouteHandler,
        middlewares: Vec<String>,
    ) {
        self.router.register(path, method, func, middlewares);
    }

    pub fn listen(&self) -> Result<(), Box<dyn Error>> {
        println!("Server started on port {}", self.port);
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        // let pool = ThreadPool::new(var("APP_THREADS").unwrap().parse().unwrap());
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let buf_reader = BufReader::new(&stream);
            let mut request = Request::parse(buf_reader);
            let response = Response::new(stream);

            let mut session: SessionBackend;
            let new_id = uuid();
            let session_id = request.cookies.get("session_id").unwrap_or(&new_id);
            if var("SESSION_DRIVER").unwrap() == "file" {
                session = SessionBackend::File(FileSession::new(session_id));
            } else if var("SESSION_DRIVER").unwrap() == "cookie" {
                session = SessionBackend::Cookie(CookieSession::new(session_id));
            } else {
                session = SessionBackend::NoSession(NoSession::new());
            }
            session.init(&request).unwrap();
            request.session = session;

            println!("{} request at {}", request.method, request.path);
            self.router.invoke(request, response);

            // pool.execute(|| {
            //     let buf_reader = BufReader::new(&stream);
            //     let request = Request::parse(buf_reader);
            //     println!("{} request at {}", request.method, request.path);
            //     self.router.invoke(request, Response::new(stream));
            // });
        }
        Ok(())
    }
}

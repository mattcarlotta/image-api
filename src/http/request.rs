use super::{Method, Response, StatusCode};
use std::fs;
use std::str;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Request<'a> {
    pub status_code: StatusCode,
    pub method: Method,
    pub body: String,
    pub path: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(buffer: &'a [u8]) -> Response {
        let body = String::new();
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        let parsed_req = req.parse(buffer).unwrap();

        let path = req.path.unwrap();
        let method = Method::from_str(req.method.unwrap()).unwrap();

        if parsed_req.is_partial() {
            return Response::new(StatusCode::BadRequest, method, path, body);
        };

        // TODO Hand this off to Router
        let (status_code, filename) = match path {
            "/" => (StatusCode::Ok, "hello.html"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                (StatusCode::Ok, "hello.html")
            }
            _ => (StatusCode::NotFound, "404.html"),
        };

        let body = match fs::read_to_string(filename) {
            Ok(b) => b,
            Err(_) => body,
        };

        Response::new(status_code, method, path, body)
    }
}

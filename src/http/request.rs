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
    /// Attempts to parse a buffer stream
    ///
    /// Arguments:
    /// * buffer: &[u8: 2]
    ///
    /// Returns a `Response` that contains a `status_code`, `method` and `path`
    pub fn parse(buffer: &'a [u8]) -> Response {
        let body = fs::read_to_string("400.html").unwrap();
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        let (path, method) = match req.parse(buffer) {
            Ok(r) => {
                let path = match req.path {
                    Some(p) => p,
                    None => "",
                };
                let method = match req.method {
                    Some(m) => m,
                    None => "",
                };
                let method = Method::from_str(method).unwrap();
                if r.is_partial() || path.is_empty() || method == Method::INVALIDMETHOD {
                    return Response::new(StatusCode::NotImplemented, method, path, body);
                };

                (path, method)
            }
            Err(_) => {
                return Response::new(StatusCode::BadRequest, Method::INVALIDMETHOD, "", body)
            }
        };

        // TODO Hand this off to RouteHandler
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

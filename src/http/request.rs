use super::Method;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        // attempt to parse the path and method from the incoming request header
        let (path, method) = match req.parse(buffer) {
            Ok(r) => {
                let path = req.path.unwrap_or("");
                let method = req.method.unwrap_or("");
                let mut method = Method::from_str(method).unwrap();

                // if the request/path are invalid sets method to invalid
                // which will be caught in RouterHandler delegater
                if r.is_partial() || path.is_empty() {
                    method = Method::INVALIDMETHOD;
                }

                (path, method)
            }
            Err(_) => ("", Method::INVALIDMETHOD),
        };

        Request { method, path }
    }
}

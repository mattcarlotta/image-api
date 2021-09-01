use chrono::{DateTime, Utc};

use super::Method;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub timestamp: DateTime<Utc>,
}

impl<'a> Request<'a> {
    /// Parses headers from the incoming request buffer
    ///
    /// Arguments:
    /// * buffer: [u8]
    ///
    pub fn new(buffer: &'a [u8], timestamp: DateTime<Utc>) -> Self {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        // attempt to parse the path and method from the incoming request header
        let (path, method) = match req.parse(buffer) {
            Ok(r) => {
                let path = req.path.unwrap_or("");
                let method = req.method.unwrap_or("");
                let mut method = Method::from_str(method).unwrap();

                // if the request/path are invalid sets method to invalid
                // which will be caught in the Router
                if r.is_partial() || path.is_empty() {
                    method = Method::INVALIDMETHOD;
                }

                (path, method)
            }
            Err(_) => ("", Method::INVALIDMETHOD),
        };

        Request {
            method,
            path,
            timestamp,
        }
    }
}

use super::Method;
use crate::utils::normalize_path;
use chrono::{DateTime, Utc};
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub timestamp: DateTime<Utc>,
}

impl<'a> Request {
    /// Parses headers from the incoming request buffer
    ///
    /// Arguments:
    /// * buffer: &[u8]
    /// * timestamp: DateTime<Utc>
    ///
    pub fn new(buffer: &'a [u8], timestamp: DateTime<Utc>) -> Self {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        let mut path = String::new();
        let mut method = Method::Invalidmethod;
        // attempt to parse the path and method from the incoming request header
        if let Ok(r) = req.parse(buffer) {
            let parsed_path = req.path.unwrap_or("");
            let parsed_method = req.method.unwrap_or("");
            method = Method::from_str(parsed_method).unwrap();

            // if the request/path are invalid sets method to invalid
            // which will be caught in the main controller
            if r.is_partial() || parsed_path.is_empty() {
                method = Method::Invalidmethod;
            }

            // normalizes path to strip extra slashes
            path = format!("{}", normalize_path(parsed_path).display());
        };

        Request {
            method,
            path,
            timestamp,
        }
    }
}

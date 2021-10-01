use super::{AllowedHosts, Method};
use crate::utils::normalize_path;
use chrono::{DateTime, Utc};
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub allowed_host: String,
    pub method: Method,
    pub path: String,
    pub timestamp: DateTime<Utc>,
}

impl<'a> Request {
    /// Parses headers from the incoming request buffer
    ///
    /// Arguments:
    /// * buffer: &[u8]
    /// * allowedhosts: AllowedHosts
    /// * timestamp: DateTime<Utc>
    ///
    pub fn new(buffer: &'a [u8], allowedhosts: AllowedHosts, timestamp: DateTime<Utc>) -> Self {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);

        let mut path = String::new();
        let mut method = Method::Invalidmethod;
        let mut allowed_host = allowedhosts[1].to_string();

        // attempt to parse the path and method from the incoming request header
        if let Ok(r) = req.parse(buffer) {
            let parsed_path = req.path.unwrap_or("");
            let parsed_method = req.method.unwrap_or("");

            // parse headers and compare if requester host matches an allowed hostname
            for header in req.headers {
                if header.name == "Host" {
                    for host in allowedhosts.iter() {
                        let requester = String::from_utf8_lossy(header.value);
                        if host.as_str() == &requester {
                            allowed_host = requester.to_string();
                        }
                    }
                }
            }

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
            allowed_host,
            method,
            path,
            timestamp,
        }
    }
}

use super::Method;
use chrono::{DateTime, Utc};
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
    /// * buffer: &[u8]
    /// * timestamp: DateTime<Utc>
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
                // which will be caught in the main controller
                if r.is_partial() || path.is_empty() {
                    method = Method::Invalidmethod;
                }

                (path, method)
            }
            Err(_) => ("", Method::Invalidmethod),
        };

        Request {
            method,
            path,
            timestamp,
        }
    }
}


#[cfg(test)]
mod test {
    use super::{Method, Request};
    use chrono::prelude::Utc;

    #[test]
    fn parse_good_request() {
        let buf = b"GET /hello.html HTTP/1.1\r\n\r\n";
        let date = Utc::now();
        let req = Request::new(buf, date);

        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, "/hello.html");
        assert_eq!(req.timestamp, date);
    }

    #[test]
    fn parse_bad_request() {
        let buf = b"POST / HTTP/1.1\r\n";
        let date = Utc::now();
        let req = Request::new(buf, date);

        assert_eq!(req.method, Method::Invalidmethod);
        assert_eq!(req.path, "/");
        assert_eq!(req.timestamp, date);
    }
} 

use super::{Method, StatusCode};

#[derive(Debug)]
pub struct Response<'a> {
    pub status_code: StatusCode,
    pub method: Method,
    body: String,
    pub path: &'a str,
}

impl<'a> Response<'a> {
    pub fn new(status_code: StatusCode, method: Method, path: &'a str, body: String) -> Self {
        Self {
            status_code,
            method,
            body,
            path,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.response(),
            self.body.len(),
            self.body
        )
    }
}

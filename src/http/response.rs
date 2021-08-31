use super::{Method, StatusCode};
use chrono::prelude::{DateTime, Utc};
use std::io::prelude::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response<'a> {
    pub status_code: StatusCode,
    pub method: Method,
    body: String,
    pub path: &'a str,
}

impl<'a> Response<'a> {
    /// Stores the result of a parsed Request
    ///
    /// Arguments:
    /// * status_code: StatusCode
    /// * method: Method
    /// * path: &'a str
    /// * body: String
    ///
    pub fn new(status_code: StatusCode, method: Method, path: &'a str, body: String) -> Self {
        Self {
            status_code,
            method,
            body,
            path,
        }
    }

    /// Attempts to write a `Response` to a `TcpStream` and logs the result to the terminal
    ///
    /// Arguments:
    /// * stream: TcpStream
    /// * timestamp:  DateTime<Utc>
    ///
    pub fn send(&self, mut stream: TcpStream, timestamp: DateTime<Utc>) {
        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.parse(),
            self.body.len(),
            self.body
        );

        stream.write_all(response.as_bytes()).unwrap();

        println!(
            "[{}] - {} {} HTTP/1.1 {} {}ms",
            timestamp.format("%B %d %Y, %I:%M:%S %P"),
            self.method,
            self.path,
            self.status_code,
            (Utc::now() - timestamp).num_milliseconds()
        );

        stream.flush().unwrap();
    }
}

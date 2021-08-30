use super::{Method, StatusCode};
use chrono::prelude::{DateTime, Utc};
use std::io::Write;

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

    /// Attempts to write a Response to a TcpStream and logs the result to the terminal
    ///
    /// Arguments:
    /// * stream: &mut impl Write
    /// * timestamp:  DateTime<Utc>
    ///
    pub fn send(mut self, stream: &mut impl Write, timestamp: DateTime<Utc>) -> Result<(), String> {
        let error = match write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.parse(),
            self.body.len(),
            self.body
        ) {
            Ok(_) => None,
            Err(e) => Some(e.to_string()),
        };

        // set status to 500 if the response fails to be written
        if !error.is_none() {
            self.status_code = StatusCode::ServerError
        };

        println!(
            "[{}] - {} {} HTTP/1.1 {} {}ms",
            timestamp.format("%B %d %Y, %I:%M:%S %P"),
            self.method,
            self.path,
            self.status_code,
            (Utc::now() - timestamp).num_milliseconds()
        );

        if !error.is_none() {
            return Err(error.unwrap());
        }

        Ok(())
    }
}

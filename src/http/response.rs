use super::{ContentType, Method, StatusCode};
use chrono::prelude::{DateTime, Utc};
use std::io::prelude::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub enum ResponseBody {
    Chunked(Vec<u8>),
    Text(String),
}

impl ResponseBody {
    /// Parses a `ResponseBody` to a tuple of: `Vec<u8>` and `String`
    pub fn parse(self) -> (Vec<u8>, String) {
        match self {
            ResponseBody::Chunked(body) => {
                (body.to_owned(), "Transfer-Encoding: chunked".to_string())
            }
            ResponseBody::Text(body) => (
                body.to_owned().into_bytes(),
                format!("Content-Length: {}", body.len()),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Response<'a> {
    status_code: StatusCode,
    method: Method,
    content_type: ContentType,
    path: &'a str,
    headers: String,
    body: Vec<u8>,
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
    pub fn new(
        status_code: StatusCode,
        method: Method,
        path: &'a str,
        content_type: ContentType,
        body: ResponseBody,
    ) -> Self {
        let (body, headers) = ResponseBody::parse(body);

        Self {
            status_code,
            method,
            path,
            content_type,
            headers,
            body,
        }
    }

    /// Attempts to write a `Response` to a `TcpStream` and logs the result to the terminal
    ///
    /// Arguments:
    /// * stream: TcpStream
    /// * timestamp:  DateTime<Utc>
    ///
    pub fn send(&self, mut stream: TcpStream, timestamp: DateTime<Utc>) {
        let header = format!("HTTP/1.1 {} {}", self.status_code, self.status_code.parse());
        let date = format!("Date: {}", Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"));
        let ct = format!("Content-Type: {}", self.content_type);

        let mut response = [
            &header,
            "Server: rustybuckets/0.0.1",
            &date,
            &ct,
            self.headers.as_str(),
            "X-Content-Type-Options: nosniff",
            "X-Frame-Options: DENY",
            "\r\n",
        ]
        .join("\r\n")
        .into_bytes();

        //let mut response = format!(
        //    "HTTP/1.1 {} {}\r\nServer: rustybuckets/0.0.1\r\nDate: {}\r\nContent-Type:{}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\n{}\r\n\r\n",
        //    self.status_code,
        //    self.status_code.parse(),
        //    Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"),
        //    self.content_type,
        //    self.headers,
        //).into_bytes();

        response.extend(&self.body);

        stream.write_all(&response).unwrap();

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

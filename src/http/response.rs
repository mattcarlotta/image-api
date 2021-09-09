use super::{ContentType, Method, Request, StatusCode};
use chrono::prelude::{DateTime, Utc};
use std::io::prelude::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub enum ResponseType {
    Chunked(Vec<u8>),
    Text(String),
}

impl ResponseType {
    /// Parses a `ResponseType` to a tuple of: `Vec<u8>` and `String`
    pub fn parse(self) -> (Vec<u8>, String) {
        match self {
            ResponseType::Chunked(body) => (body, "Transfer-Encoding: chunked".to_string()),
            ResponseType::Text(body) => (
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
    timestamp: DateTime<Utc>,
    stream: &'a mut TcpStream,
}

impl<'a> Response<'a> {
    /// Stores the result of a parsed Request
    ///
    /// Arguments:
    /// * req: Request
    /// * stream: &mut TcpStream
    ///
    pub fn new(req: &Request<'a>, stream: &'a mut TcpStream) -> Self {
        Self {
            status_code: StatusCode::Ok,
            method: req.method,
            path: req.path,
            content_type: ContentType::Html,
            timestamp: req.timestamp,
            stream,
        }
    }

    /// Replaces the default `content_type` with `Content-Type` from a string slice
    ///
    /// Arguments:
    /// s: &str
    ///
    pub fn set_content(mut self, s: &str) -> Self {
        self.content_type = ContentType::from_extension(s).unwrap_or(ContentType::Invalid);

        self
    }

    /// Replaces `status_code` with `StatusCode` from a `u16` integer
    ///
    /// Arguments:
    /// * i: u16
    ///
    pub fn set_status(mut self, i: u16) -> Self {
        self.status_code = StatusCode::convert(i);

        self
    }

    /// Attempts to write a `Response` to a `TcpStream` and logs the result to the terminal
    ///
    /// Arguments:
    /// * body: ResponseType
    ///
    pub fn send(self, body: ResponseType) {
        let (body, headers) = ResponseType::parse(body);

        let header = format!("HTTP/1.1 {} {}", self.status_code, self.status_code.parse());
        let date = format!("Date: {}", Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"));
        let ct = format!("Content-Type: {}", self.content_type);

        let mut response = [
            &header,
            "Server: rustybuckets/0.0.1",
            &date,
            &ct,
            headers.as_str(),
            "X-Content-Type-Options: nosniff",
            "X-Frame-Options: DENY",
            "\r\n",
        ]
        .join("\r\n")
        .into_bytes();

        response.extend(&body);

        self.stream.write_all(&response).unwrap();

        println!(
            "[{}] - {} {} HTTP/1.1 {} {}ms",
            self.timestamp.format("%B %d %Y, %I:%M:%S %P"),
            self.method,
            self.path,
            self.status_code,
            (Utc::now() - self.timestamp).num_milliseconds()
        );

        self.stream.flush().unwrap()
    }
}

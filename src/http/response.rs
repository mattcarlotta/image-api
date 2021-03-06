#![allow(dead_code)]
use super::{ContentType, Method, Request, StatusCode};
use chrono::prelude::{DateTime, Utc};
use std::env;
use std::io::prelude::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub enum ResponseType {
    Chunked(Vec<u8>),
    Html(String),
    Css(String),
    Text(String),
}

impl ResponseType {
    /// Parses a `ResponseType` to a tuple of: `Vec<u8>` and `String`
    pub fn parse(self) -> (Vec<u8>, String) {
        match self {
            ResponseType::Chunked(body) => (
                body,
                "Transfer-Encoding: chunked\r\nCache-Control: public, max-age=31536000, immutable"
                    .to_string(),
            ),
            ResponseType::Html(body) | ResponseType::Text(body) | ResponseType::Css(body) => (
                body.to_owned().into_bytes(),
                format!("Content-Length: {}", body.len()),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Response<'a> {
    allowed_host: String,
    status_code: StatusCode,
    method: Method,
    content_type: ContentType,
    path: String,
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
    pub fn new(req: &Request, stream: &'a mut TcpStream) -> Self {
        Self {
            allowed_host: req.allowed_host.to_owned(),
            status_code: StatusCode::Ok,
            method: req.method,
            path: req.path.to_owned(),
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
        let (body, data_type) = ResponseType::parse(body);

        let header = format!("HTTP/1.1 {} {}", self.status_code, self.status_code.parse());
        let date = format!("Date: {}", Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"));
        let ct = format!("Content-Type: {}", self.content_type);
        let allowed_host = format!("Access-Control-Allow-Origin: {}", self.allowed_host);
        let in_testing = env::var("testing").unwrap_or_else(|_| "".to_string());
        let allow_script = if !in_testing.is_empty() {
            "'self' 'unsafe-inline' 'unsafe-eval'"
        } else {
            "none"
        };
        let csp = format!("Content-Security-Policy: default-src 'none'; font-src 'none'; script-src {}; manifest-src 'none'; media-src 'none'; style-src 'self'; base-uri 'none'; img-src 'self' *.mattcarlotta.sh data:; form-action 'none'; frame-ancestors 'none'; connect-src 'none'; worker-src 'none';", allow_script);

        let mut response = [
            &header,
            "Server: rustybuckets/1.0.0",
            &date,
            &ct,
            &data_type,
            "Connection: keep-alive",
            "Content-Disposition: inline",
            &csp,
            &allowed_host,
            "Access-Control-Allow-Methods: GET, OPTIONS",
            "Strict-Transport-Security: max-age=15768000; includeSubDomains",
            "X-Content-Type-Options: nosniff",
            "X-Frame-Options: DENY",
            "\r\n",
        ]
        .join("\r\n")
        .into_bytes();

        response.extend(&body);

        self.stream
            .write_all(&response)
            .expect("Failed to write body to stream");

        println!(
            "[{}] - {} {} HTTP/1.1 {} {}ms",
            self.timestamp.format("%B %d %Y, %I:%M:%S %P"),
            self.method,
            self.path,
            self.status_code,
            (Utc::now() - self.timestamp).num_milliseconds()
        );

        self.stream.flush().expect("Failed to flush stream")
    }
}

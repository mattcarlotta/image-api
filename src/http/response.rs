use super::{ContentType, Method, ResponseBody, StatusCode};
use chrono::prelude::{DateTime, Utc};
use std::io::prelude::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response<'a> {
    pub status_code: StatusCode,
    pub method: Method,
    pub content_type: ContentType,
    body: ResponseBody,
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
    pub fn new(
        status_code: StatusCode,
        method: Method,
        path: &'a str,
        content_type: ContentType,
        body: ResponseBody,
    ) -> Self {
        Self {
            status_code,
            method,
            body,
            content_type,
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
        let body = match &self.body {
            ResponseBody::Chunked(value) => value.clone(),
            _ => vec![0; 24],
        };

        let response = format!(
               "HTTP/1.1 {} {}\r\nServer: rustybuckets/0.0.1\r\nDate: {}\r\nContent-Type:{}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nTransfer-Encoding: chunked\r\n\r\n",
                self.status_code,
                self.status_code.parse(),
                Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"),
                self.content_type,
                // body.len(),
              //  body
            );

        let mut r = response.to_string().into_bytes();
        r.extend(body);

        // let headers = [
        // "HTTP/1.1 200 OK",
        // "Content-type: image/png",
        // "Transfer-Encoding: chunked",
        //   "\r\n",
        // ];
        //let mut response = headers.join("\r\n").to_string().into_bytes();
        //response.extend(body);

        //match stream.write(&response) {
        //    Ok(_) => println!("Response sent"),
        //    Err(e) => println!("Failed sending response: {}", e),
        // }

        stream.write_all(&r).unwrap();

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

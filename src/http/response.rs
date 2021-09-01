use super::{ContentType, Method, StatusCode};
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

#[derive(Debug)]
pub enum ResponseBody {
    Chunked(Vec<u8>),
    Text(String),
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
        let commonheaders = format!(
            "HTTP/1.1 {} {}\r\nServer: rustybuckets/0.0.1\r\nDate: {}\r\nContent-Type:{}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\n",
            self.status_code,
            self.status_code.parse(),
            Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"),
            self.content_type
        );

        let response = match &self.body {
            ResponseBody::Chunked(body) => {
                let mut response =
                    format!("{}Transfer-Encoding: chunked\r\n\r\n", commonheaders).into_bytes();

                response.extend(body);

                response
            }
            ResponseBody::Text(body) => {
                let response = format!(
                    "{}Content-Length: {}\r\n\r\n{}",
                    commonheaders,
                    body.len(),
                    body
                );

                response.into_bytes()
            }
        };

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

// let response = format!(
//        "HTTP/1.1 {} {}\r\nServer: rustybuckets/0.0.1\r\nDate: {}\r\nContent-Type:{}\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nTransfer-Encoding: chunked\r\n\r\n",
//         self.status_code,
//         self.status_code.parse(),
//         Utc::now().format("%a, %d %b %Y %H:%M:%S GMT"),
//         self.content_type,
//         // body.len(),
//       //  body
//     );

// let mut r = response.to_string().into_bytes();
// r.extend(body);

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

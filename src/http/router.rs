use crate::http::Request;
use chrono::prelude::Utc;
use std::io::Read;
use std::net::TcpStream;

pub struct Router;

impl Router {
  /// Attempts to parse `TcpStream` as a `Request` and sends back a `Response` to the client
  ///
  /// Arguments:
  /// * stream: TcpStream
  ///
  pub fn handle_request(mut stream: TcpStream) {
    let timestamp = Utc::now();
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = Request::parse(&buffer);

    if let Err(e) = response.send(&mut stream, timestamp) {
      panic!("Failed to send response: {}", e);
    }
  }
}

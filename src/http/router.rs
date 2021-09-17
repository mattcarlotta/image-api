use super::{Method, Request, Response};
use crate::controllers;
use crate::lrucache::Cache;
use chrono::prelude::Utc;
use std::io::Read;
use std::net::TcpStream;

/// Attempts to parse a `TcpStream` to a `Request` and `Response`
/// and sends back a `Response` to the client
///
/// Arguments:
/// * stream: mut TcpStream
/// * cache: Cache
///
#[allow(clippy::unused_io_amount)]
pub fn controller(mut stream: TcpStream, cache: Cache) {
    let timestamp = Utc::now();
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let req = Request::new(&buffer, timestamp);

    let res = Response::new(&req, &mut stream);

    if !req.method.is_valid() {
        return controllers::badrequest(req, res);
    }

    // matches requests via requested method and path
    match req.method {
        Method::Get => match req.path.as_str() {
            "/" => controllers::hello(req, res),
            "/favicon.ico" => controllers::favicon(req, res),
            "/testing" => controllers::testing(req, res),
            _ => controllers::image(cache, req, res),
        },
        _ => controllers::badrequest(req, res),
    }
}

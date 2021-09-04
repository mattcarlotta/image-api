use super::{Method, Request, Response};
use crate::controllers;
use chrono::prelude::Utc;
use std::io::Read;
use std::net::TcpStream;

pub struct Router;

impl Router {
    /// Attempts to parse a `TcpStream` to a `Request` and `Response`
    /// and sends back a `Response` to the client
    ///
    /// Arguments:
    /// * stream: mut TcpStream
    ///
    pub fn controller(mut stream: TcpStream) {
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
            Method::Get => match req.path {
                "/" => controllers::hello(req, res),
                "/sleep" => controllers::sleep(req, res),
                _ => controllers::image(req, res),
            },
            _ => controllers::badrequest(req, res),
        }
    }
}

use super::{bad_req_file, Method, StatusCode};
use std::fs;
use std::thread;
use std::time::Duration;

pub struct RouteHandler;

impl RouteHandler {
    /// Handles incoming requests and delegates them to specific route controllers
    ///
    /// Arguments:
    /// * method: &Method
    /// * path: &str
    ///
    /// Returns a tuple of `(status_code, body)`
    pub fn delegater<'a>(method: &Method, path: &'a str) -> (StatusCode, String) {
        // TODO Create route controllers to reduce this boilerplate
        let (status_code, filename) = match method {
            Method::GET => match path {
                "/" => (StatusCode::Ok, "hello.html"),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));
                    (StatusCode::Ok, "hello.html")
                }
                _ => (StatusCode::NotFound, "404.html"),
            },
            _ => (StatusCode::NotFound, "404.html"),
        };

        let body = match fs::read_to_string(filename) {
            Ok(b) => b,
            Err(_) => bad_req_file(),
        };

        (status_code, body)
    }
}

use super::{file_not_found, server_error_file, ContentType, Method, ResponseBody, StatusCode};
use chunked_transfer::Encoder;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
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
    pub fn delegater<'a>(
        method: &Method,
        path: &'a str,
    ) -> (StatusCode, ContentType, ResponseBody) {
        // TODO Create route controllers to reduce this boilerplate
        let (status_code, content_type, body) = match method {
            Method::GET => match path {
                "/" => {
                    let body = fs::read_to_string("hello.html").unwrap();

                    (StatusCode::Ok, ContentType::HTML, ResponseBody::Text(body))
                }
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));

                    let body = fs::read_to_string("hello.html").unwrap();

                    (StatusCode::Ok, ContentType::HTML, ResponseBody::Text(body))
                }
                _ => {
                    // TODO - Make sure requested image size doesn't extend beyond actual image dimensions
                    // open requested image
                    let mut existing_file = match File::open("placeholder2.png") {
                        Ok(file) => file,
                        Err(_) => {
                            return (StatusCode::NotFound, ContentType::HTML, file_not_found())
                        }
                    };

                    // read the contents of the image
                    let mut buf = Vec::new();
                    match existing_file.read_to_end(&mut buf) {
                        Ok(vec) => vec,
                        Err(reason) => {
                            println!("Unable to read the contents of the image: {}", reason);

                            return (
                                StatusCode::ServerError,
                                ContentType::HTML,
                                server_error_file(),
                            );
                        }
                    };

                    let mut body = Vec::new();
                    {
                        let mut encoder = Encoder::with_chunks_size(&mut body, 8192);
                        encoder.write_all(&buf).unwrap();
                    }

                    (
                        StatusCode::Ok,
                        ContentType::PNG,
                        ResponseBody::Chunked(body),
                    )
                }
            },
            _ => (StatusCode::NotFound, ContentType::HTML, file_not_found()),
        };

        (status_code, content_type, body)
    }
}

impl Display for ResponseBody {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

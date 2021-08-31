use super::{bad_req_file, ContentType, Method, StatusCode};
use chunked_transfer::Encoder;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

pub struct RouteHandler;

#[derive(Debug)]
pub enum ResponseBody {
    Chunked(Vec<u8>),
    Str(String),
}

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
                //"/" => (StatusCode::Ok, ContentType::HTML, "hello.html"),
                //"/sleep" => {
                //    thread::sleep(Duration::from_secs(5));
                //    (StatusCode::Ok, ContentType::HTML, "hello.html")
                //}
                _ => {
                    // TODO - Make sure requested image size doesn'p extend beyond actual image dimensions
                    // open requested image
                    let mut existing_file = match File::open("placeholder.png") {
                        Ok(file) => file,
                        Err(reason) => {
                            //return Err(format!("Unable to open image: {}", reason).to_string());
                            println!("Unable to open image: {}", reason);
                            unimplemented!()
                        }
                    };

                    // read the contents of the image
                    let mut buf = Vec::new();
                    match existing_file.read_to_end(&mut buf) {
                        Ok(vec) => vec,
                        Err(reason) => {
                            println!("Unable to read the contents of the image: {}", reason);
                            unimplemented!();
                            // return Err("Resource was not found.".to_string());
                        }
                    };

                    let mut body = Vec::new();
                    {
                        let mut encoder = Encoder::with_chunks_size(&mut body, 1024);
                        encoder.write_all(&buf).unwrap();
                    }

                    (
                        StatusCode::Ok,
                        ContentType::PNG,
                        ResponseBody::Chunked(body),
                    )
                }
            },
            _ => {
                let body = match fs::read_to_string("404.html") {
                    Ok(b) => ResponseBody::Str(b),
                    Err(_) => bad_req_file(),
                };
                (StatusCode::NotFound, ContentType::HTML, body)
            }
        };

        (status_code, content_type, body)
    }
}

impl Display for ResponseBody {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

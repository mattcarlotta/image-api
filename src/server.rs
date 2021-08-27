use crate::connections::Scheduler;
use crate::logger::logger;

// use std::convert::TryFrom;
use chrono::prelude::Utc;
// use chrono::Duration;
use httparse;
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

//pub trait RouteHandler {
//    fn handle_request(&mut self, request: )
//}
//
#[derive(Debug)]
pub struct Server<'a> {
    address: &'a str,
    port: usize,
}

impl<'a> Server<'a> {
    // Creates a single Tcp Server
    //
    // Arguments:
    // * address: &str
    // * port: usize
    //
    pub fn new(address: &'a str, port: usize) -> Self {
        Server { address, port }
    }

    // Binds a TcpListener to a host, creates a connection pool and hands off requests to Router
    pub fn listen(&self) -> Result<(), &str> {
        let host = format!("{}:{}", self.address, self.port);

        println!("Listening for requests on: {}", &host);

        let listener = TcpListener::bind(host).unwrap();
        // TODO Change this hardcoded number to arg
        let pool = Scheduler::new(8)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => pool.create(|| {
                    handle_request(stream);
                }),
                Err(e) => println!("Unable to handle request: {}", e),
            }
        }

        Ok(())
    }
}

// TODO Move this into it a Router
fn handle_request(mut stream: TcpStream) {
    let date = Utc::now();
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);
    let parsed_req = req.parse(&buffer).unwrap();

    if parsed_req.is_partial() {
        // TODO This should default to 404 not found
        panic!("The request is invalid!");
    };

    let path = req.path.unwrap();
    let method = req.method.unwrap();
    let version: &str = if req.version.unwrap() == 1 {
        "1.1"
    } else {
        "2"
    };

    let (status, filename) = match path {
        "/" => ("200 OK", "hello.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "hello.html")
        }
        _ => ("404 Not Found", "404.html"),
    };
    let body = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/{} {}\r\nContent-Length: {}\r\n\r\n{}",
        &version,
        &status,
        body.len(),
        body,
    );

    stream.write(response.as_bytes()).unwrap();

    let diff = Utc::now() - date;

    logger(
        date,
        &method,
        &path,
        &version,
        &status,
        diff.num_milliseconds(),
    );

    stream.flush().unwrap();
}

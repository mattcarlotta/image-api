use crate::connections::Scheduler;
// use std::convert::TryFrom;
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
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let body = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body,
    );

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}

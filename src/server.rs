use crate::connections::TaskPool;
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
pub struct Server {
    address: &'static str,
    port: usize,
}

impl Server {
    pub fn new(address: &'static str, port: usize) -> Self {
        Server { address, port }
    }

    pub fn listen(&self) {
        let host = format!("{}:{}", self.address, self.port);

        println!("Listening for requests on: {}", &host);

        let listener = TcpListener::bind(host).unwrap();
        // TODO Change this hardcoded number to arg
        let pool = match TaskPool::new(8) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.create(|| {
                handle_request(stream);
            })
        }
    }
}

// TODO Move this into it a RouteHandler
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

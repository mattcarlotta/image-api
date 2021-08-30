use crate::connections::Scheduler;
use crate::http::Request;
use crate::logger::logger;

// use std::convert::TryFrom;
use chrono::prelude::Utc;
// use chrono::Duration;
// use httparse;
// use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
// use std::thread;
// use std::time::Duration;

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

    let response = Request::new(&buffer);

    stream.write_all(response.format().as_bytes()).unwrap();

    let diff = Utc::now() - date;

    logger(
        date,
        response.method,
        response.path,
        response.status_code,
        diff.num_milliseconds(),
    );

    stream.flush().unwrap();
}

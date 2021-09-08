macro_rules! relative {
    ($path:expr) => {
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
    };
}

extern crate chrono;
extern crate chunked_transfer;
extern crate httparse;
extern crate image;
extern crate num_cpus;

use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

mod connections;
mod controllers;
mod http;
mod lrucache;
mod reqimage;
mod utils;

fn main() {
    let address = env::var("host").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("port").unwrap_or_else(|_| "5000".to_string());
    let host = format!("{}:{}", address, port);

    println!("Listening for requests on: {}", &host);

    let listener = TcpListener::bind(host).unwrap();
    let scheduler = connections::Scheduler::new();
    let init_cache = Arc::new(Mutex::new(lrucache::LRUCache::<String, Vec<u8>>::new(50)));

    for stream in listener.incoming() {
        let cache = Arc::clone(&init_cache);
        match stream {
            Ok(stream) => scheduler.create(|| {
                http::controller(stream, cache);
            }),
            Err(e) => println!("Unable to handle request: {}", e),
        }
    }
}

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

use server::Server;
use std::env;
use std::process::exit;

mod connections;
mod controllers;
mod http;
mod reqimage;
mod server;
mod utils;

fn main() {
    let host = env::var("host").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("port").unwrap_or_else(|_| |_| "5000".to_string());

    if let Err(e) = Server::new(host, port).listen() {
        println!("Server has encountered an error: {}", e);
        exit(1);
    };
}

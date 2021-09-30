extern crate chrono;
extern crate chunked_transfer;
extern crate httparse;
extern crate image;
extern crate libwebp_sys;
extern crate num_cpus;
use std::env;

macro_rules! relative {
    ($path:expr) => {
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
    };
}

mod connections;
mod controllers;
mod http;
mod lrucache;
mod reqimage;
mod utils;

#[cfg(test)]
mod tests;

fn main() {
    let address = env::var("host").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("port").unwrap_or_else(|_| "5000".to_string());
    let hostname = format!("{}:{}", address, port);

    http::Server::new(hostname).listen();
}

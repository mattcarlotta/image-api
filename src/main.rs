extern crate chrono;
extern crate chunked_transfer;
extern crate httparse;
extern crate image;
extern crate num_cpus;
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

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

fn main() {
    let address = env::var("host").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("port").unwrap_or_else(|_| "5000".to_string());
    let host = format!("{}:{}", address, port);

    println!("Listening for requests on: {}", &host);

    let listener = TcpListener::bind(host).unwrap();
    let scheduler = connections::Scheduler::new();
    let init_cache = Arc::new(Mutex::new(lrucache::LRUCache::new(50)));

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

#[cfg(test)]
mod test {
    use reqwest::blocking::get;
    use reqwest::header::{CONTENT_TYPE, TRANSFER_ENCODING};

    #[test]
    #[ignore]
    fn e2e_integrations() {
        let hello_res = get("http://localhost:5000").unwrap();
        assert!(hello_res.status().is_success());
        assert_eq!(
            hello_res.headers().get(CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let notfound_res = get("http://localhost:5000/123").unwrap();
        assert!(!notfound_res.status().is_success());
        assert_eq!(
            notfound_res.headers().get(CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );

        let img_res = get("http://localhost:5000/placeholder.png").unwrap();
        assert!(img_res.status().is_success());
        assert_eq!(img_res.headers().get(CONTENT_TYPE).unwrap(), "image/png");
        assert_eq!(img_res.headers().get(TRANSFER_ENCODING).unwrap(), "chunked");
    }
}

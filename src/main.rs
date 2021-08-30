extern crate httparse;

extern crate chrono;

use std::process::exit;

// use router::Router;
use server::Server;
//use std::env;

mod connections;
mod http;
//mod router;
mod server;

fn main() {
    //let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    //let path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    // TODO Pull address and port from env or fallback
    if let Err(e) = Server::new("127.0.0.1", 5000).listen() {
        println!("Server has encountered an error: {}", e);
        exit(1);
    };
}

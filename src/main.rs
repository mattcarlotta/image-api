// use router::Router;
use server::Server;
//use std::env;

mod connections;
//mod router;
mod server;

fn main() {
    //let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    //let path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1", 5000);

    server.listen();
}

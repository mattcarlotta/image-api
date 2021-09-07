use crate::connections::Scheduler;
use crate::http::Router;
use crate::lrucache::LRUCache;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub struct Server {
    address: String,
    port: String,
}

impl Server {
    /// Creates a single TcpServer
    ///
    /// Arguments:
    /// * address: String
    /// * port: String
    ///
    pub fn new(address: String, port: String) -> Self {
        Server { address, port }
    }

    /// Binds the TcpListener to a host, creates a connection scheduler and hands off requests to `Router`
    pub fn listen(&self) -> Result<(), &str> {
        let host = format!("{}:{}", self.address, self.port);

        println!("Listening for requests on: {}", &host);

        let listener = TcpListener::bind(host).unwrap();
        let scheduler = Scheduler::new();
        let c = Arc::new(Mutex::new(LRUCache::<String, Vec<u8>>::new(50)));

        for stream in listener.incoming() {
            let cache = Arc::clone(&c);
            match stream {
                Ok(stream) => scheduler.create(|| {
                    Router::controller(stream, cache);
                }),
                Err(e) => println!("Unable to handle request: {}", e),
            }
        }

        Ok(())
    }
}

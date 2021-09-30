use super::router;
use crate::connections::Scheduler;
use crate::lrucache::LRUCache;
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub type Client = Arc<str>;

pub struct Server {
    client: String,
    hostname: String,
}

impl Server {
    /// Initialize a new TcpServer:
    ///
    /// Arguments:
    ///
    /// * hostname - String
    ///
    pub fn new(hostname: String) -> Self {
        let client = env::var("client").unwrap_or_else(|_| hostname.to_string());

        Server { client, hostname }
    }

    /// Listens for incoming requests to hostname
    pub fn listen(&self) {
        let listener = TcpListener::bind(&self.hostname).unwrap();
        let scheduler = Scheduler::new();
        let init_cache = Arc::new(Mutex::new(LRUCache::new(50)));
        let host = Arc::from(self.client.as_str());

        println!(
            "Listening for requests to hostname: {} from client: {}",
            &self.hostname, &self.client
        );

        for stream in listener.incoming() {
            let cache = Arc::clone(&init_cache);
            let hostname = Arc::clone(&host);
            match stream {
                Ok(stream) => scheduler.create(|| {
                    router(stream, cache, hostname);
                }),
                Err(e) => println!("Unable to handle request: {}", e),
            }
        }
    }
}

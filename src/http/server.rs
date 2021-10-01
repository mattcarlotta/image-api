use super::router;
use crate::connections::Scheduler;
use crate::lrucache::LRUCache;
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

pub type AllowedHosts = Arc<Vec<String>>;

pub struct Server {
    allowedhosts: Vec<String>,
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
        let client = env::var("client").unwrap_or_else(|_| "localhost:3000".to_string());
        let mut allowedhosts = Vec::with_capacity(2);
        allowedhosts.push(hostname.to_string());
        allowedhosts.push(client.to_string());

        Server {
            allowedhosts,
            client,
            hostname,
        }
    }

    /// Listens for incoming requests to hostname
    pub fn listen(&self) {
        let listener = TcpListener::bind(&self.hostname).unwrap();
        let scheduler = Scheduler::new();
        let init_cache = Arc::new(Mutex::new(LRUCache::new(50)));
        let host = Arc::from(self.allowedhosts.clone());

        println!(
            "Listening for requests to hostname: {} from hostname and client: {}",
            &self.hostname, &self.client
        );

        for stream in listener.incoming() {
            let cache = Arc::clone(&init_cache);
            let allowedhosts = Arc::clone(&host);
            match stream {
                Ok(stream) => scheduler.create(|| {
                    router(stream, cache, allowedhosts);
                }),
                Err(e) => println!("Unable to handle request: {}", e),
            }
        }
    }
}

use crate::connections::Scheduler;
use crate::http::Router;
use std::net::TcpListener;

#[derive(Debug)]
pub struct Server<'a> {
    address: &'a str,
    port: usize,
}

impl<'a> Server<'a> {
    /// Creates a single TcpServer
    ///
    /// Arguments:
    /// * address: &str
    /// * port: usize
    ///
    pub fn new(address: &'a str, port: usize) -> Self {
        Server { address, port }
    }

    /// Binds the TcpListener to a host, creates a connection scheduler and hands off requests to `Router`
    pub fn listen(&self) -> Result<(), &str> {
        let host = format!("{}:{}", self.address, self.port);

        println!("Listening for requests on: {}", &host);

        let listener = TcpListener::bind(host).unwrap();
        // TODO Change this hardcoded number to arg/num of cpus
        let scheduler = Scheduler::new(8)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => scheduler.create(|| {
                    Router::controller(stream);
                }),
                Err(e) => println!("Unable to handle request: {}", e),
            }
        }

        Ok(())
    }
}

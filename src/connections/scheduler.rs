use super::Worker;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

pub type Task = Box<dyn FnOnce() + Send + 'static>;
pub type PendingTasks = Arc<Mutex<Receiver<Signal>>>;

// Communicates the current operational status to the Worker
pub enum Signal {
    CreateTask(Task),
    TerminateTask,
}

pub struct Scheduler {
    workers: Vec<Worker>,
    tx: Sender<Signal>,
}

impl Scheduler {
    // Initializes a connection pool to hand-off requests to a Worker
    //
    // Arguments:
    // * channels: usize
    //
    pub fn new<'a>(channels: usize) -> Result<Self, &'a str> {
        if channels <= 0 {
            return Err("You must specify the maximum number of channels to create.");
        }

        let (tx, rx) = channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(channels);

        for id in 0..channels {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Ok(Scheduler { workers, tx })
    }

    // Generates a new task by sending it to a Worker
    pub fn create<T>(&self, t: T)
    where
        T: FnOnce() + Send + 'static,
    {
        self.tx.send(Signal::CreateTask(Box::new(t))).unwrap();
    }
}

// Drops channels when an error occurs
impl Drop for Scheduler {
    fn drop(&mut self) {
        // send signal to workers to terminate their task
        for _ in &self.workers {
            self.tx.send(Signal::TerminateTask).unwrap();
        }

        // take ownership of channels and join them
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(channel) = worker.channel.take() {
                channel.join().unwrap();
            }
        }
    }
}

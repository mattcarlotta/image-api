use super::Worker;
use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

pub type Task = Box<dyn FnOnce() + Send + 'static>;

pub enum Signal {
    CreateTask(Task),
    TerminateTask,
}

pub struct TaskPool {
    workers: Vec<Worker>,
    tx: Sender<Signal>,
}

impl TaskPool {
    pub fn new(channels: usize) -> Result<Self, &'static str> {
        if channels <= 0 {
            return Err("You must specify the maximum number of channels");
        }

        let (tx, rx) = channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(channels);

        for id in 0..channels {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        Ok(TaskPool { workers, tx })
    }

    pub fn create<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let t = Box::new(f);

        self.tx.send(Signal::CreateTask(t)).unwrap();
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        // send signal to workers to terminate task
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

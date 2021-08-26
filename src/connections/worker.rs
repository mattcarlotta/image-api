use super::Signal;
use std::sync::{mpsc::Receiver, Arc, Mutex};
use std::thread::{spawn, JoinHandle};

pub struct Worker {
    pub id: usize,
    pub channel: Option<JoinHandle<()>>,
}

impl Worker {
    // Accepts tasks from the Scheduler and conditionally invokes them based upon their Signal
    pub fn new(id: usize, rx: Arc<Mutex<Receiver<Signal>>>) -> Self {
        let channel = spawn(move || loop {
            let signal = rx.lock().unwrap().recv().unwrap();
            match signal {
                Signal::CreateTask(t) => t(),
                Signal::TerminateTask => break,
            }
        });

        Worker {
            id,
            channel: Some(channel),
        }
    }
}

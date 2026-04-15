use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle}, time::Duration,
};

pub struct ThreadPool {
    pub workers: Vec<Worker>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize, rx : Arc<Mutex<Receiver<Job>>>) -> Self {
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let rx = Arc::clone(&rx);

        for id in 1..=size {
                let rx = Arc::clone(&rx);
                let worker = Worker::new(id, rx);
                workers.push(worker);
        }

        return Self { workers};
    }
}

pub struct Worker {
    pub id: usize,
    pub thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move|| {
            loop {
                let job = rx.lock().unwrap().recv_timeout(Duration::from_millis(5000));
                match job {
                    Ok(job) => {
                        job()
                    },
                    Err(_) => {

                    }
                }
            }
        });
        Self { id, thread}
    }
}

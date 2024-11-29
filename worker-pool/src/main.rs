use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{mem, thread};

struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Option<i32>>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let Some(job) = receiver.lock().unwrap().recv().unwrap() else {
                println!("Worker {} exited", id);
                break;
            };
            println!("Worker {} received job {}", id, job);
        });

        Self { thread }
    }
}

struct WorkerPool {
    workers: Vec<Worker>,
    sender: Sender<Option<i32>>,
}

impl WorkerPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        Self { workers, sender }
    }

    fn execute(&self, job: i32) {
        self.sender.send(Some(job)).unwrap()
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        // send all workers a signal to exit
        for _ in 0..self.workers.len() {
            self.sender.send(None).unwrap()
        }
        // wait for all workers to exit
        for worker in mem::take(&mut self.workers) {
            worker.thread.join().unwrap();
        }
    }
}

fn main() {
    let pool = WorkerPool::new(5);
    for job in 0..8 {
        pool.execute(job);
    }
}

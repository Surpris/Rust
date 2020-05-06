//! thread_test
//!
//! provide functions for multi threading

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// Trait to move the contents in Box
trait FnBox {
    fn call_box(self: Box<Self>);
}


/// Implementation of FnBox for functions with FnOnce
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

/// Job type
type Job = Box<dyn FnBox + Send + 'static>;

/// Enum including Job
enum Message {
    NewJob(Job),
    Terminate,
}

/// ThreadPool struct
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/// Basic implementation for ThreadPool
impl ThreadPool {
    /// Create a new ThreadPool instance.
    ///
    /// size: the number of workers in the pool
    ///
    /// # panic
    ///
    /// if size == 0 then the `new` method will panic.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/// Implementation of Drop for ThreadPool
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // println!("Shuting down all workers.");
        for worker in &mut self.workers {
            // println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Worker struct
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// Basic implementation for Worker
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        // println!("Worker {} got a job; executing.", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        // println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

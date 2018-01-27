use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::time::Duration;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Creates a new thread pool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if size is zero or below.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        // Send the job
        self.sender.send(job).unwrap();
    }

    pub fn join_all(self) {
        for worker in self.workers {
            worker.join()
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
               // Look for jobs for 100 ms, if there's no action
               // break out of the loop and finish thread execution
               let job = match receiver.lock().unwrap().recv_timeout(Duration::from_millis(100)) {
                   Ok(job) => job,
                   Err(_) => {
                       break;
                   }
               };


               // Execute the closure from execute
               job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }

    // Interface to allow calling thread to await execution of
    // workers
    fn join(self) {
        self.thread.join();
    }
}
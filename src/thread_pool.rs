use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::time::Duration;
use nbt_packet::NetBiosPacket;

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
        where F: (FnOnce() -> Option<NetBiosPacket>) + Send + 'static
    {
        let job = Box::new(f);

        // Send the job
        self.sender.send(job).unwrap();
    }

    pub fn join_all(self) -> Vec<NetBiosPacket> {
        let mut results: Vec<NetBiosPacket> = Vec::with_capacity(255);
        for worker in self.workers {
            results.append(&mut worker.join());
        }
        results
    }
}

trait FnBox {
    fn call_box(self: Box<Self>) -> Option<NetBiosPacket>;
}

impl<F: FnOnce() -> Option<NetBiosPacket>> FnBox for F {
    fn call_box(self: Box<F>) -> Option<NetBiosPacket> {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<Vec<NetBiosPacket>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let mut thread_results: Vec<NetBiosPacket> = Vec::with_capacity(4);
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
               match job.call_box() {
                   Some(packet) => thread_results.push(packet),
                   _ => ()
               }
            }
            thread_results
        });

        Worker {
            id,
            thread,
        }
    }

    // Interface to allow calling thread to await execution of
    // workers
    fn join(self) -> Vec<NetBiosPacket> {
        self.thread.join().unwrap()
    }
}
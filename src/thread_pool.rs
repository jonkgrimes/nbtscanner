use nbt_packet::NetBiosPacket;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::Vec;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> Option<NetBiosPacket> + Send + 'static,
    {
        let job = Box::new(f);

        // Send the job
        self.sender.send(Message::Process(job)).unwrap();
    }

    pub fn stop(&self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate);
        }
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

enum Message {
    Process(Job),
    Terminate,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            let mut thread_results: Vec<NetBiosPacket> = Vec::with_capacity(4);
            loop {
                let message = match receiver.lock().unwrap().recv() {
                    Ok(message) => message,
                    Err(_) => {
                        break;
                    }
                };

                // Execute the closure from execute
                match message {
                    Message::Process(job) => {
                        if let Some(packet) = job.call_box() {
                            thread_results.push(packet);
                        }
                    }
                    Message::Terminate => break,
                }
            }
            thread_results
        });

        Worker { id, thread }
    }

    // Interface to allow calling thread to await execution of
    // workers
    fn join(self) -> Vec<NetBiosPacket> {
        self.thread.join().unwrap()
    }
}

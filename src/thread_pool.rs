use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    job_count: Arc<AtomicUsize>,
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

        let job_count = Arc::new(AtomicUsize::new(0));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&job_count)));
        }

        ThreadPool {
            workers,
            sender,
            job_count,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        // Send the job
        self.sender.send(job).unwrap();
        
        // Increment the number of jobs
        self.job_count.fetch_add(1, Ordering::Acquire);
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, job_count: Arc<AtomicUsize>) -> Worker {
        let thread = thread::spawn(move || {
            println!("Worker thread {} spawned!", id);
            loop {
               // Get the number of jobs
               let count = job_count.load(Ordering::Relaxed);
               println!("job_count = {}", count);
               if count == 0 {
                   println!("job_count == 0, exiting thread {}", id);
                   break;
               }
               
               println!("Looking for jobs");
               // Looks like there's some in the queue, grab the next one
               let job = receiver.lock().unwrap().recv().unwrap();


               // Execute the closure from execute
               job.call_box();

               // Decrement the jobs count
               job_count.fetch_sub(1, Ordering::Acquire);
            }
        });

        Worker {
            id,
            thread,
        }
    }

    fn join(self) {
        self.thread.join();
    }
}
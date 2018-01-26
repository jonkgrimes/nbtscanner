use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    job_count: Arc<Mutex<u8>>,
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

        let job_count = Arc::new(Mutex::new(0));

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
        let mut count = self.job_count.lock().unwrap();
        *count +=1 ;
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, job_count: Arc<Mutex<u8>>) -> Worker {
        let thread = thread::spawn(move || {
            println!("Worker thread {} spawned!", id);
            loop {
               // Get the number of jobs
               let mut count = job_count.lock().unwrap();
               if *count <= 0 { // No more jobs, exit
                 break;
               }
               
               // Looks like there's some in the queue, grab the next one
               let job = receiver.lock().unwrap().recv().unwrap();

               // Execite the closure from execute
               job.call_box();

               // Decrement the jobs count
               *count -= 1;
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
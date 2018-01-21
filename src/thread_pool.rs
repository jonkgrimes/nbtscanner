use std::thread;
use std::vec::Vec;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>
}

impl ThreadPool {
    /// Creates a new thread pool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if size is zero or below.
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size as usize); 

        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {

    }
}
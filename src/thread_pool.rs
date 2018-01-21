pub struct ThreadPool {

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

        ThreadPool {}
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {

    }
}
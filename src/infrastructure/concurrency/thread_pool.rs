//! Thread Pool Manager (dummy)

pub struct ThreadPool;

impl ThreadPool {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        job();
    }
}

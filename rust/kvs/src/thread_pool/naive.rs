use crate::thread_pool::ThreadPool;
use super::Result;
use std::thread;

pub struct NaiveThreadPool {
    size: u32
}

impl ThreadPool for NaiveThreadPool {
    fn new(size: u32) -> Result<Self> {
        Ok(NaiveThreadPool {
            size
        })
    }

    fn spawn<T>(&self, f: T) where T: FnOnce() + Send + 'static {
        thread::spawn(f);
    }
}
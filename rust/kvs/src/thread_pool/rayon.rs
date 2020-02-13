use rayon::prelude::*;
use crate::{ThreadPool, KvError};

pub struct RayonThreadPool {
    pool: rayon::ThreadPool,
}

impl ThreadPool for RayonThreadPool {
    fn new(size: u32) -> Result<Self, KvError> where Self: Sized {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(size as usize).build()?;
        Ok(
            RayonThreadPool {
                pool
            }
        )
    }

    fn spawn<T>(&self, f: T) where T: FnOnce() + Send + 'static {
        self.pool.spawn(f)
    }
}


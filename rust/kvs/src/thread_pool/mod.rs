pub use super::Result;

mod naive;

pub use self::naive::NaiveThreadPool;

pub trait ThreadPool {
    fn new(size: u32) -> Result<Self>
        where Self: Sized;

    fn spawn<T>(&self, f: T)
        where T: FnOnce() + Send + 'static;
}
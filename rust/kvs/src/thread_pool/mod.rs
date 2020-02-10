pub use super::Result;

mod naive;
mod share_queue;

extern crate crossbeam;

type Func = Box<dyn FnOnce() + Send + 'static>;

enum ThreadPoolMessage {
    RunJob(Func),
    Shutdown,
}

pub use self::naive::NaiveThreadPool;
pub use self::share_queue::SharedQueueThreadPool;

pub trait ThreadPool {
    fn new(size: u32) -> Result<Self>
        where Self: Sized;

    fn spawn<T>(&self, f: T)
        where T: FnOnce() + Send + 'static;
}
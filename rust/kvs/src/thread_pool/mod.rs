pub use super::Result;

mod naive;
mod rayon;
mod share_queue;

type Func = Box<dyn FnOnce() + Send + 'static>;


pub const NAIVE_THREAD_POOL_NAME: &str = "naive";
pub const SHARED_QUEUE_THREAD_POOL_NAME: &str = "sharequeue";
pub const RAYON_THREAD_POOL_NAME: &str = "rayon";

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
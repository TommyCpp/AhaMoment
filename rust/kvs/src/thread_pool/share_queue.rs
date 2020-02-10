use crossbeam::channel::{Sender, Receiver};
use crate::thread_pool::{Func, ThreadPool};
use crate::KvError;
use std::thread;


//implement a single sender, multi receiver schema.
//Start n thread when created, those thread will in infinite loop and wait until receive some message and act accordingly.
pub struct SharedQueueThreadPool {
    tx: Sender<Func>
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(size: u32) -> Result<Self, KvError> where Self: Sized {
        let (sender, receiver) = crossbeam::channel::unbounded::<Func>();
        for _ in 0..size {
            let re = receiver.clone();
            let task_runner = TaskRunner(re);
            thread::spawn(move || {
                run_task(task_runner)
            });
        };
        let thread_pool = SharedQueueThreadPool {
            tx: sender
        };
        Ok(thread_pool)
    }

    fn spawn<T>(&self, f: T) where T: FnOnce() + Send + 'static {
        self.tx.send(Box::new(f)).expect("no thread in thread pool")
    }
}

struct TaskRunner(Receiver<Func>);


fn run_task(task_runner: TaskRunner) {
    loop {
        match task_runner.0.recv() {
            Ok(task) => {
                task();
            }
            Err(err) => {
                debug!("Error when running task, see {}", err)
            }
        }
    }
}
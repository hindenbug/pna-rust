use crate::Result;

mod naive;

pub use self::naive::NaiveThreadPool;

pub trait ThreadPool {
    /// Creates a thread pool
    ///
    /// return error if failed to create any thread
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

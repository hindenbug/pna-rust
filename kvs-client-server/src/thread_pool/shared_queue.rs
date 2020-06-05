use crate::{Result, ThreadPool};
use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};

use std::thread;

pub struct SharedQueueThreadPool(Sender<Box<dyn FnOnce() + Send + 'static>>);
#[derive(Clone)]
struct TaskReceiver(Receiver<Box<dyn FnOnce() + Send + 'static>>);

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> {
        let (sender, recv) = unbounded::<Box<dyn FnOnce() + Send + 'static>>();
        for _ in 0..threads {
            let r_clone = TaskReceiver(recv.clone());
            thread::spawn(move || {
                while let Ok(job) = r_clone.0.recv() {
                    job();
                }
            });
        }

        Ok(SharedQueueThreadPool(sender))
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.send(Box::new(job)).unwrap();
    }
}

impl Drop for TaskReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let r_clone = self.clone();
            thread::spawn(move || {
                while let Ok(job) = r_clone.0.recv() {
                    job();
                }
            });
        }
    }
}

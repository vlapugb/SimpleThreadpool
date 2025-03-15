use crossbeam::channel::*;
use std::fmt::*;
use std::thread;

type Job<R> = Box<dyn FnOnce() -> R + Send + 'static>;
pub struct ThreadPool<R> {
    deq: Sender<Job<R>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<R> ThreadPool<R>
where
    R: Debug + Send + 'static,
{
    pub fn new(size: usize) -> ThreadPool<R> {
        let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
        let (tx, rx): (Sender<Job<R>>, Receiver<Job<R>>) = unbounded();
        for thread in 0..size {
            let rx_clone = rx.clone();

            workers.push(thread::spawn(move || {
                let copy_thread = thread;
                while let Ok(job) = rx_clone.recv() {
                    println!("Thread {} got job!", copy_thread);
                    job();
                }

                println!("Worker #{} shutting down", copy_thread);
            }));
        }

        ThreadPool { deq: tx, workers }
    }
    pub fn enqueue<F>(&self, f: F) -> Receiver<R>
    where
        F: FnOnce() -> R + Send + 'static,
    {
        let (_result_tx, result_rx): (Sender<R>, Receiver<R>) = unbounded();
        let job = Box::new(f);
        self.deq.send(job).unwrap();
        result_rx
    }

    pub fn dispose(self) {
        drop(self.deq);
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}

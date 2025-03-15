use std::thread;
use std::thread::Builder;
use queue::*;
use std::collections::*;
use std::sync::*;
use std::fmt::*;
use crossbeam::channel::*;
type Job<R> = Box<dyn FnOnce() -> R  + Send + 'static>;

pub struct ThrdPool<R> {
    deq: crossbeam::channel::Sender<Job<R>>,
    workers: Vec<thread::JoinHandle<()>>,
    poolsize: usize,
}


impl<R> ThrdPool<R> where
    R: Debug + Send + 'static,
{
    pub fn new(size: usize) -> ThrdPool<R> {
        let mut workers: Vec<thread::JoinHandle<()>> = Vec::new();
        let (tx, rx): (Sender<Job<R>>, Receiver<Job<R>>) = unbounded();
        for thrd in 0..size {
            let rxclone = rx.clone();
            
            let res = workers.push(thread::spawn( move || {
                let thrde = thrd;
                while let Ok(job) = rxclone.recv() {
                    println!("Thread {} got job!", thrde);
                    let res = job();
                }
                
                println!("Worker #{} shutting down", thrde);
            }
            ));
            
        }
        
        ThrdPool {
            deq: tx,
            workers,
            poolsize: size,
        }
        
    }
    pub fn enqueue<F>(&self, f: F) -> Receiver<R>
    where 
        F: FnOnce() -> R + Send + 'static {
        let (res_tx, res_rx): (Sender<R>, Receiver<R>) = unbounded();
        let job = Box::new(f);
        self.deq.send(job).unwrap();
        res_rx
    }
    
    pub fn dispose(self) {
        drop(self.deq);
        for w in self.workers {
            w.join().unwrap();
        }
    }

}
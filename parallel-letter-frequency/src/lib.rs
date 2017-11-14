use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

struct ThreadPool<S,T> {
    jobs: mpsc::Sender<S>,
    results: mpsc::Receiver<T>,
    threads: Vec<thread::JoinHandle<()>>,
}

pub fn frequency(lines: &[&str], _: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for line in lines {
        for c in line.to_lowercase().chars() {
            if c.is_alphabetic() { *result.entry(c).or_insert(0) += 1; }
        }
    }
    result
}

impl <S,T> ThreadPool<S,T> 
    where S: Send {
    fn new<F>(f: F, n_workers: usize) -> ThreadPool<S,T>
        where 
            F: FnOnce() + Send + 'static {
        let (job_tx, job_rx) = mpsc::channel::<S>();
        let (result_tx, result_rx) = mpsc::channel::<T>();
        let mut pool: ThreadPool<S,T> = ThreadPool { 
            jobs: job_tx,
            results: result_rx,
            threads: Vec::with_capacity(n_workers),
        };
        let thread_rx = Arc::new(job_rx);
        for _ in 0..n_workers {
            pool.threads.push(thread::spawn(|| {
            }));
        }
        pool
    }
}

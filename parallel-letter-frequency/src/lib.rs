use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

pub fn frequency(lines: &[&str], _: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let (tx, rx) = mpsc::channel::<&str>();
    for line in lines {
        for c in line.to_lowercase().chars() {
            if c.is_alphabetic() { *result.entry(c).or_insert(0) += 1; }
        }
    }
    result
}

impl ThreadPool {
    fn new<F>(f: F, n_workers: usize) -> ThreadPool 
        where 
            F: FnOnce() + Send + 'static {
        let mut pool = ThreadPool{ threads: Vec::with_capacity(n_workers) };
        for _ in 0..n_workers {
            pool.threads.push(thread::spawn(|| {}));
        }
        pool
    }
}

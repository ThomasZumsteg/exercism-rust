use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, mpsc};

pub fn frequency(lines: &[&str], n_workers: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    println!("Creating pool");
    let pool = ThreadPool::new(n_workers, frequency_job);
    println!("Done");
    let jobs = lines.iter().map(|s| s.to_string());
    println!("Starting jobs");
    for job in pool.do_jobs(jobs) { 
        println!("Merging {:?}", job);
        result.merge(job);
    }
    result
}

fn frequency_job(line: String) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for c in line.to_lowercase().chars() {
        if c.is_alphabetic() { *result.entry(c).or_insert(0) += 1 }
    }
    result
}

trait Merge<T> {
    fn merge(&mut self, other: T);
}

impl <K,V> Merge<HashMap<K,V>> for HashMap<K,V>
    where K: std::cmp::Eq + std::hash::Hash,
          V: std::ops::AddAssign {
    fn merge(&mut self, other: HashMap<K,V>) {
        for (k, v) in other {
            match self.entry(k) {
                Entry::Occupied(o) => { *o.into_mut() += v },
                Entry::Vacant(o) => { o.insert(v); }
            };
        }
    }
}

struct ThreadPool<S,T> 
{
    f: fn(S) -> T,
    threads: Vec<thread::JoinHandle<()>>,
    queue: Vec<mpsc::Sender<S>>,
    rx: mpsc::Receiver<T>,
}

impl <'a,S,T> ThreadPool<S,T>
    where
        S: Debug + Send + 'static,
        T: Debug + Send + 'static,
{
    fn new(n_workers: usize, f: fn(S) -> T) -> ThreadPool<S,T> {
        let (results_tx, results_rx) = mpsc::channel::<T>();
        let mut pool = ThreadPool { 
            f: f,
            threads: Vec::new(),
            queue: Vec::new(),
            rx: results_rx,
        };
        for i in 0..n_workers {
            let (jobs_tx, jobs_rx) = mpsc::channel::<S>();
            let thread_results = results_tx.clone();
            pool.threads.push(thread::spawn(move || {
                while let Ok(item) = jobs_rx.recv() {
                    println!("Thread {}: Doing work", i);
                    thread_results.send(f(item));
                }
            }));
            pool.queue.push(jobs_tx);
        }
        pool
    }

    fn do_jobs<I>(&self, jobs: I) -> &Self
        where 
            I: IntoIterator<Item=S> {
        println!("Doing jobs");
        let (tx, rx) = mpsc::channel();
        for job in jobs {
            tx.send((self.f)(job)); 
        }
        self
    }
}

impl <S,T> Iterator for ThreadPool<S,T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        unimplemented!()
    }
}


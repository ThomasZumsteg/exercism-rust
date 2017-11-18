use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub fn frequency(lines: &[&str], n_workers: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let pool = ThreadPool::new(n_workers, frequency_job);
    for job in pool.do_jobs(lines.to_vec()) { result.merge(job); }
    result
}

fn frequency_job(line: &str) -> HashMap<char, usize> {
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

struct ThreadPool<S,T> {
    f: fn(S) -> T,
    threads: Vec<thread::JoinHandle<usize>>,
}

struct ThreadPoolIterator<T> {
    rx: mpsc::Receiver<T>,
}

impl <S,T> IntoIterator for ThreadPool<S,T> {
    type Item = T;
    type IntoIter = ThreadPoolIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

impl <S,T> ThreadPool<S,T> {
    fn new(n_workers: usize, f: fn(S) -> T) -> ThreadPool<S,T> {
        let mut pool = ThreadPool { f: f, threads: Vec::new() };
        pool
    }

    fn do_jobs<I>(&self, jobs: I) -> ThreadPoolIterator<T>
        where 
            I: IntoIterator<Item=S> {
        let (tx, rx) = mpsc::channel();
        for job in jobs {
            tx.send((self.f)(job)); 
        }
        ThreadPoolIterator { rx: rx }
    }
}

impl <T> Iterator for ThreadPoolIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Ok(result) = self.rx.recv() { Some(result) }
        else { None }
    }
}

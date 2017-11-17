use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub fn frequency(lines: &[&str], n_workers: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let pool = MapPool::new(n_workers, frequency_job);
    for job in pool.do_work(lines) { result.merge(job); }
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
                Entry::Occupied(mut o) => { *o.into_mut() += v },
                Entry::Vacant(mut o) => { o.insert(v); }
            };
        }
    }
}

struct MapPool<S, T> {
    inputs: Vec<S>,
    results: mpsc::Receiver<T>
}

impl <S, T> MapPool <S, T> {
    fn new<F>(n_workers: usize, f: F) -> MapPool<S, T>
        where
            F: Fn(S) -> T + Sync + 'static {
        unimplemented!()
    }

    fn do_work<I>(self, work: I) -> Iterator<Item=T>
        where
            I: IntoIterator<Item=S> {
        unimplemented!()
    }
}

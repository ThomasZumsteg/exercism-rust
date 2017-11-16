use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::thread;
use std::sync;

pub fn frequency(lines: &[&str], n_workers: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let pool = MapPool::new(n_workers, &frequency_job);
    for line in lines { result.merge(frequency_job(line)); }
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

struct MapPool;

impl MapPool {
    fn new<S,T>(workers: usize, f: &Fn(S) -> T) {
        // Create channels to send and recieve jobs from
        let (jobs_tx, jobs_rx) = sync::mpsc::channel::<S>();
        let jobs_queue = sync::Arc::new(sync::Mutex::new(&jobs_rx));
        let (result_tx, result_rx) = sync::mpsc::channel::<T>();
        // Create the workers
        for _ in 0..workers {
            let this_jobs = sync::Arc::clone(&jobs_queue);
            thread::spawn(|| {
            });
        }
    }

    fn exit(&self) {
        // Quit all the workers
        unimplemented!()
    }
}

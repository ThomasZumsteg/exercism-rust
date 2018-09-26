use std::thread;
use std::sync::mpsc;
use std::collections::HashSet;

static NTHREADS: u64 = 3;

pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut pool = Vec::new();
    let (tx, rx) = mpsc::channel::<Palindrome>();
    for t in 0..NTHREADS {
        let thread_tx = tx.clone();
        pool.push(Some(thread::spawn(move || {
            let mut i = min + t;
            while i <= max {
                for j in i..max+1 {
                    let product = i * j;
                    let digits = format!("{}", product);
                    if digits.chars().into_iter().eq(digits.chars().into_iter().rev()) {
                        thread_tx.send(product).unwrap();
                    } 
                }
                i += NTHREADS;
            }
        })));
    }
    
    for handle in &mut pool {
        if let Some(thread) = handle.take() {
            thread.join().unwrap();
        }
    }
    drop(tx);
    
    let mut products = HashSet::new();
    for p in rx {
        products.insert(p);
    }
    products.iter().map(|&p| p).collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.into_iter().fold(None, |min, &p| match min {
        None => Some(p),
        Some(q) => Some(if p < q { p } else { q })
    })
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.into_iter().fold(None, |min, &p| match min {
        None => Some(p),
        Some(q) => Some(if p < q { q } else { p })
    })
}

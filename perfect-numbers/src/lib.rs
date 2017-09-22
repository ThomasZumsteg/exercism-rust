use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}

pub fn classify(num: u64) -> Result<Classification, &'static str> {
    if num <= 0 { return Err("Number must be positive") }
    let sum = factors(num).iter().fold(0, |sum, &n| sum + n);
    match num.cmp(&sum) {
        Ordering::Equal => Ok(Classification::Perfect),
        Ordering::Less => Ok(Classification::Abundant),
        Ordering::Greater => Ok(Classification::Deficient),
    }
}

fn factors(num: u64) -> HashSet<u64> {
    let mut factors = HashSet::new();
    if num == 1 { return factors; }
    factors.insert(1);
    for n in 2.. {
        if n * n > num { break }
        if num % n == 0 {
            factors.insert(n);
            factors.insert(num / n);
        }
    }
    return factors
}

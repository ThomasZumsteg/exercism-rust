use std::cmp::Ordering;
use std::fmt::Debug;

pub fn find<T: Ord + Debug>(haystack:&[T], needle:T) -> Option<usize> {
    let (mut start, mut end) = (0, haystack.len());
    while start <= end {
        let middle = (end + start) / 2;
        println!("{}:{}:{} -> {:?}:{:?}", start, middle, end, needle, haystack);
        if haystack.len() <= middle { return None }
        match needle.cmp(haystack.get(middle).unwrap()) {
            Ordering::Less => { 
                if middle <= 0 { return None }
                else { end = middle - 1; }},
            Ordering::Greater => { start = middle + 1; },
            Ordering::Equal => return Some(middle)
        }
    }
    None
}

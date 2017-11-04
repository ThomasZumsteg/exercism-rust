use std::cmp::Ordering;

pub fn find<I, T>(items:I, needle:T) -> Option<usize> 
    where I: AsRef<[T]>, T: Ord {
    let haystack = items.as_ref();
    let (mut start, mut end) = (0, haystack.len());
    while start <= end {
        let middle = (end + start) / 2;
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

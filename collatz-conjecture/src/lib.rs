use std::collections::HashSet;

pub fn collatz(n: u64) -> Result<u64, &'static str> {  
    let mut seen: HashSet<u64> = HashSet::new();
    let mut current = n;
    for i in 0.. {
        if seen.contains(&current) { break }
        else if current == 1 { return Ok(i) }

        seen.insert(current);

        if current % 2 == 0 { current /= 2 }
        else { current = 3 * current + 1 }
    }
    Err("Infinite Loop")
}

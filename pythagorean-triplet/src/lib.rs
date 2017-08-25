pub fn find() -> Option<u64> {
    let result = find_in_range(1, 1000);
    match result.len() {
        0 => None,
        _ => Some((result[0].0 * result[0].1 * result[0].2)),
    }
}

pub fn find_in_range(start: u64, stop: u64) -> Vec<(u64,u64,u64)> {
    let mut c: u64;
    let mut output = Vec::new();
    for a in start..(stop/3) {
        for b in (a+1)..((stop-a)/2) {
            c = stop - a - b;
            if a * a + b * b == c * c { 
                output.push((a,b,c));
                break;
            }
        }
    }
    output
}

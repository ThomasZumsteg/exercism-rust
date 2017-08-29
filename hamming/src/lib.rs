pub fn hamming_distance(left: &str, right: &str) -> Result<usize, &'static str> {
    if right.len() != left.len() { return Err("Not the same length") }
    Ok(left.chars()
           .zip(right.chars())
           .filter( |&(l, r)| l != r)
           .count())
}

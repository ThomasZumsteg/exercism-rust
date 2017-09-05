use std::cmp;

pub fn lsp(digits: &str, window: usize) -> Result<usize, &str> {
    if digits == "" && window == 0 { return Ok(1) }
    else if window > digits.len() { return Err("Windows longer than digits") }

    let mut result = 0;
    for i in 0..(digits.len() - window + 1) {
        let mut total = 1;
        for c in digits[i..(i+window)].chars() {
            match c.to_digit(10) {
                Some(n) => total *= n,
                None => return Err("Not a valid digit"),
            }
        }
        result = cmp::max(result, total);
    }
    Ok(result as usize)
}

pub fn series(_digits: &str, _len: usize) -> Vec<String> {
    let mut result = Vec::new();
    let n_digits = _digits.len();
    if n_digits < _len { return result }
    for i in 0..(n_digits - _len + 1) {
        result.push((_digits[i..(i+_len)]).to_string());
    }
    result
}

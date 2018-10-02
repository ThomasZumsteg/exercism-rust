extern crate rand;

use rand::Rng;

pub fn encode(key: &str, clear_text: &str) -> Option<String> {
    let shift_fn = |ch, k| (ch + k - 2 * ('a' as u8)) % 26 + ('a' as u8);
    return shift(key, clear_text, shift_fn);
}

pub fn decode(key: &str, cipher_text: &str) -> Option<String> {
    let shift_fn = |ch, k| (ch + 26 - k) % 26 + ('a' as u8);
    return shift(key, cipher_text, shift_fn);
}

pub fn encode_random(plain_text: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = (0..100)
        .map(|_| (rng.gen_range(0, 26) + ('a' as u8)) as char)
        .collect();
    let cipher_text = encode(&key.as_str(), plain_text);
    return (key, cipher_text.unwrap());
}

fn shift(key: &str, text: &str, shift_fn: fn(u8, u8) -> u8) -> Option<String> {
    if key.len() <= 0 || key.chars().any(|c| !c.is_ascii_lowercase()) { return None };
    let mut result: Vec<char> = Vec::new();
    for (ch, k) in text.chars().zip(key.chars().cycle()) {
        result.push(shift_fn(ch as u8, k as u8) as char);
    }
    return Some(result.iter().collect());
}

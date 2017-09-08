use std::ascii::AsciiExt;

pub fn encrypt(plaintext: &str) -> String {
    if plaintext == "" { return "".to_string() }
    let chars = plaintext.to_lowercase().chars()
        .filter(|c| c.is_ascii() && c.is_alphabetic())
        .collect::<Vec<char>>();
    let mut ciphertext = String::new();
    let rows = (((chars.len() - 1) as f64).sqrt() as usize) + 1;
    println!("{} chars -> rows = {}", chars.len(), rows);
    for r in 0..rows {
        for c in 0..{
            if let Some(i) = chars.get(r + c * rows) { ciphertext.push(*i) }
            else { break }
        }
        if r < (rows - 1) { ciphertext.push(' ') }
    }
    ciphertext
}


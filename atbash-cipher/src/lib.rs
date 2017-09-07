use std::ascii::AsciiExt;

fn rotate_char(c: char) -> Option<char> {
    let (z, z_upper, a) = (('z' as u8), ('Z' as u8), ('a' as u8));
    match c {
        c if !c.is_ascii() || !c.is_alphanumeric() => None,
        c if c.is_lowercase() => Some((z       - (c as u8) + a) as char),
        c if c.is_uppercase() => Some((z_upper - (c as u8) + a) as char),
        c => Some(c),
    }
}

pub fn encode(plaintext: &str) -> String {
    let mut i = 0;
    let mut result = String::new();
    for c in plaintext.chars() {
        if let Some(r) = rotate_char(c) {
            result.push(r);
            if (i + 1) % 5 == 0 { result.push(' ') }
            i += 1;
        }
    }
    if result.ends_with(" ") { result.pop(); }
    result
}

pub fn decode(ciphertext: &str) -> String {
    let mut plaintext = String::new();
    for c in ciphertext.chars() {
        if let Some(p) = rotate_char(c) {
            plaintext.push(p);
        }
    }
    plaintext
}


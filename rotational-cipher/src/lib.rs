use std::ascii::AsciiExt;

fn make_rotation(dist: u8) -> Box<Fn(char) -> Option<char>> {
    Box::new(move | c | { 
        match c {
            _ if !c.is_ascii() => None,
            c if c.is_uppercase() =>
                Some(((c as u8 + dist - 'A' as u8) % 26 + 'A' as u8) as char),
            c if c.is_lowercase() => 
                Some(((c as u8 + dist - 'a' as u8) % 26 + 'a' as u8) as char),
            c => Some(c),
        }
    })
}

pub fn rotate(plaintext: &str, distance: u8) -> String {
    let f = make_rotation(distance);
    plaintext.chars().filter_map(|c| f(c)).collect::<String>()
}


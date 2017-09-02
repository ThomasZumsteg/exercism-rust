use std::collections::HashSet;
use std::iter::FromIterator;

fn ascii_lowercase() -> HashSet<char> {
    HashSet::from_iter((0..26).map(|x| (x + 'a' as u8) as char))
}

pub fn is_pangram(words: &str) -> bool {
    let letters: HashSet<char> = HashSet::from_iter(words.to_lowercase().chars());
    ascii_lowercase().is_subset(&letters)
}

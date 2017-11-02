use std::collections::HashSet;

pub fn check(word: &str) -> bool {
    let mut seen: HashSet<char> = HashSet::new();
    for c in word.to_lowercase().chars() {
        if !c.is_alphanumeric() { continue }
        if seen.contains(&c) { return false }
        seen.insert(c);
    }
    true
}

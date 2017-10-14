pub fn anagrams_for<'a>(anagram: &str, words: &[&'a str]) -> Vec<&'a str> {
    let letters = get_letters(anagram);
    words.iter().filter(|&word| {
            let w = get_letters(word);
            w.1 == letters.1 && w.0 != letters.0 })
        .map(|&w| w).collect()
}

fn get_letters(word: &str) -> (String, Vec<char>) {
    let lower = word.to_lowercase();
    let mut chars = lower.chars().collect::<Vec<char>>();
    chars.sort();
    (lower, chars)
}

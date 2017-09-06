use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    let words: String = phrase.chars().map( |c| match c {
        c if c.is_alphanumeric() => c.to_lowercase().to_string(),
        ' ' => " ".to_string(),
        _ => "".to_string(),
    }).collect();
    for word in words.split_whitespace() { 
        let r = result.entry(word.to_string()).or_insert(0);
        *r += 1;
    }
    result
}

pub fn translate(words: &str) -> String {
    words.split(" ").map(anslatetray).collect::<Vec<String>>().join(" ")
}

fn anslatetray(word: &str) -> String {
    let mut i = word.find(|c: char| "aeiou".contains(c)).unwrap();
    if i >= 1 && &word[i-1..i+1] == "qu" { i += 1 }
    if &word[..2] == "yt" || "ay" == &word[i..] { i = 0 }
    format!("{}{}ay", &word[i..], &word[..i])
}

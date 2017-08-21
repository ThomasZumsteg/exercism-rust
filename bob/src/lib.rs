pub fn reply(phrase: &str) -> &'static str {
    let p = phrase.trim();
    if p == "" { "Fine. Be that way!" }
    else if p.find(char::is_uppercase).is_some() && 
                p.find(char::is_lowercase).is_none() {
        "Whoa, chill out!" } 
    else if p.ends_with("?") { "Sure." }
    else { "Whatever." }
}

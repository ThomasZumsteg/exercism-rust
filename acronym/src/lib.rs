pub fn abbreviate(long_form: &str) -> String {
    long_form
        .split(|c: char| !c.is_alphanumeric())
        .map(|word| {
            if word == "" { "".to_string() }
            else if word.chars().all(char::is_uppercase) { word[0..1].to_string() } 
            else {
                let first = word[0..1].to_uppercase();
                let extra = word[1..].chars().filter(|c| c.is_uppercase());
                first + &extra.collect::<String>()
            }
        })
        .collect::<String>()
}

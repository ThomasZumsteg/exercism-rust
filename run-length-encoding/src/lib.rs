pub fn encode(expanded: &str) -> String {
    if expanded == "" { return String::new() }

    // Add sentinal
    let (input, mut output) = (format!("{}\0", expanded), String::new());
    let mut counter = 0;

    for (ch, pre) in input.chars().zip(input.chars().skip(1)) {
        counter += 1;
        if ch == '\0' || pre != ch { 
            if counter <= 1 { output.push(ch) }
            else { output += &format!("{}{}", counter, ch) }
            counter = 0;
        }
    }
    output
}

pub fn decode(compressed: &str) -> String {
    let (mut digits, mut output) = (String::new(), String::new());
    for c in compressed.chars() {
        if c.is_digit(10) { digits.push(c) }
        else {
            match digits.parse() {
                Ok(n) => output += &c.to_string().repeat(n),
                Err(_) => output.push(c)
            }
            digits.clear();
        }
    }
    output
}

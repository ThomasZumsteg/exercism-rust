/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut result  = 0;
    let mut i = 0;
    for n in isbn.chars().filter_map(number_or_x).rev() {
        if n == 10 && i != 0 { return false }
        i += 1;
        result += i * n;
    }
    i == 10 && result % 11 == 0
}

fn number_or_x(c: char) -> Option<u32> {
    if let Some(d) = c.to_digit(10) { Some(d) }
    else if c == 'X' { Some(10) }
    else { None }
}

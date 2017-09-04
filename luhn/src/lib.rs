pub fn is_valid(number: &str) -> bool {
    let (mut total, mut i) = (0, 0);
    for c in number.chars().rev() {
        if c == ' ' { continue }
        else if !c.is_digit(10) { return false }
        let d = c.to_digit(10).unwrap() * ((i % 2) + 1);
        total += d;
        if d > 9 { total -= 9 }
        i += 1;
    }
    i > 1 && total % 10 == 0
}

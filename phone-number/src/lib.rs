pub fn number(digits: &str) -> Option<String> {
    let mut number = digits.chars().filter(|d| d.is_digit(10)).collect::<String>();
    if number.len() == 11 && number.chars().nth(0).unwrap() == '1' { 
        number.remove(0); 
    }
    if number.len() == 10 { Some(number) }
    else { None }
}

pub fn area_code(digits: &str) -> Option<String> {
    number(digits).map(|n| (&n)[0..3].to_string())
}

pub fn pretty_print(digits: &str) -> String {
    if let Some(num) = number(digits) {
        format!("({}) {}-{}", &num[0..3], &num[3..6], &num[6..10]).to_string()
    } else {
        "invalid".to_string()
    }
}

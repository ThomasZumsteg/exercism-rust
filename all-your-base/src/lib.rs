pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if to_base <= 1 || from_base <= 1 { return Err(()) }
    if let Ok(num) = read_from(from_base, number) { Ok(write_to(to_base, num)) }
    else { Err(()) }
}

fn read_from(base: u32, digits: &[u32]) -> Result<u32, ()> {
    let mut result = 0;
    for d in digits.iter() {
        if d >= &base { return Err(()) }
        result = result * base + d;
    }
    Ok(result)
}

fn write_to(base: u32, mut number: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while number > 0 {
        result.insert(0, number % base);
        number = number / base;
    }
    result
}

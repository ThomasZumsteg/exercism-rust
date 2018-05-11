pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut remainer = num;
    let mut lenth = 0;
    while remainer > 0 {
        digits.push(remainer % 10);
        remainer /= 10;
        lenth += 1;
    }
    num == digits.iter().map(|d| d.pow(lenth)).sum()
}

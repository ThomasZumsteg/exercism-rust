pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut factor = 2;
    let mut remainer = n;
    while 1 < remainer {
        if remainer % factor == 0 {
            factors.push(factor);
            remainer /= factor;
        } else {
            factor += if factor != 2 { 2 } else { 1 };
        }
    }
    return factors;
}

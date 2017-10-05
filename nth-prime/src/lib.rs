pub fn nth(n: isize) -> Result<isize, &'static str> {
    if n <= 0 { return Err("Needs to be greater than 0") }
    let mut primes = vec![2,3,5,7];
    let mut m = 9;
    while (primes.len() as isize) < n {
        let mut prime = true;
        for p in &primes {
            if p * p > m { 
                break;
            } else if m % p == 0 {
                prime = false;
                break;
            }
        }
        if prime { primes.push(m) }
        m += 2;
    }
    Ok(primes[(n - 1) as usize])
}

pub fn nth(n: isize) -> Result<isize, &'static str> {
    if n <= 0 { return Err("Needs to be greater than 0") }
    let mut primes = Vec::new();
    primes.push(2);
    primes.push(3);
    primes.push(5);
    primes.push(7);
    let mut m = 9;
    while (primes.len() as isize) < n {
        let mut prime = true;
        for p in &mut prime {
            if *p * *p > m { 
                break;
            } else if m % *p == 0 {
                prime = false;
                break;
            }
        }
        if prime { primes.push(m) }
        m += 2;
    }
    Ok(primes[((n - 1) as usize)])
}

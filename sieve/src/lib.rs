pub fn primes_up_to(limit: usize) -> Vec<usize> {
    if limit <= 1 { return Vec::new() }
    let mut primes = vec![];
    let mut sieve = std::iter::repeat(true).take(limit+1)
        .collect::<Vec<_>>();
    for n in 2.. {
        if n > limit { break }
        if sieve[n] {
            primes.push(n);
            for m in n.. {
                if m*n > limit { break }
                sieve[m*n] = false;
            }
        }
    }
    primes
}

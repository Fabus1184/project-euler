fn prime_factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = vec![];
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

pub fn solution() -> u64 {
    prime_factors(600_851_475_143).into_iter().max().unwrap()
}

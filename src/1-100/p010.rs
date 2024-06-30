use primes::PrimeSet;

pub fn solution() -> u64 {
    primes::Sieve::new()
        .iter()
        .take_while(|&x| x < 2_000_000)
        .sum()
}

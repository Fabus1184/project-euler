use primes::PrimeSet;

pub fn solution() -> u64 {
    primes::TrialDivision::new().iter().nth(10_000).unwrap()
}

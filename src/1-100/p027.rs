fn consecutive_prime_quadratic(a: i32, b: i32) -> impl Iterator<Item = i32> {
    (0..).map(move |n| n * n + a * n + b)
}

pub fn solution() -> i32 {
    itertools::iproduct!(-1000..1000, -1000..1000)
        .max_by_key(|&(a, b)| {
            consecutive_prime_quadratic(a, b)
                .take_while(|&n| n > 0)
                .take_while(|&n| primes::is_prime(n as u64))
                .count()
        })
        .map(|(a, b)| a * b)
        .unwrap()
}

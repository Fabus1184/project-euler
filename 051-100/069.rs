use std::collections::HashSet;

fn prime_factorization(n: usize) -> HashSet<usize> {
    let mut n = n;
    let mut factors = HashSet::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            factors.insert(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    if n > 1 {
        factors.insert(n);
    }
    factors
}

fn euler_totient(n: usize) -> f64 {
    let factors = prime_factorization(n);
    n as f64
        * factors
            .iter()
            .map(|&x| 1.0 - 1.0 / x as f64)
            .product::<f64>()
}

println!(
    "{:?}",
    (2..=1_000_000)
        .map(|n| (n as f64 / euler_totient(n), n))
        .reduce(|a, b| {
            if a > b {
                a
            } else {
                b
            }
        })
        .unwrap()
        .1
);

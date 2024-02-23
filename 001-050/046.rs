#!/usr/bin/env rust-script

// cargo-deps: primal="0.3.2"

fn odd_composites() -> impl Iterator<Item = u64> {
    (3..).filter(|&x| x % 2 == 1 && !primal::is_prime(x))
}

fn is_sum_of_prime_and_twice_square(n: u64) -> bool {
    (1..)
        .map(|x| 2 * x * x)
        .take_while(|&x| x < n)
        .any(|x| primal::is_prime(n - x))
}

println!(
    "{:?}",
    odd_composites()
        .filter(|&x| !is_sum_of_prime_and_twice_square(x))
        .next()
        .unwrap()
);

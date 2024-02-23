#!/usr/bin/env rust-script

// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

fn is_lychrel(n: &BigInt) -> bool {
    let mut n = n.clone();
    for _ in 0..50 {
        n += n
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<BigInt>()
            .unwrap();
        if n.to_string() == n.to_string().chars().rev().collect::<String>() {
            return false;
        }
    }
    true
}

println!(
    "{}",
    (1..10000)
        .map(|n| BigInt::from(n))
        .filter(|n| is_lychrel(n))
        .count()
);

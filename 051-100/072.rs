#!/usr/bin/env rust-script

// cargo-deps: fraction = "0.13.1"

use fraction::BigInt;
use fraction::ToPrimitive;

use std::collections::HashSet;

type F = fraction::GenericFraction<BigInt>;

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

fn euler_totient(n: usize) -> F {
    let factors = prime_factorization(n);
    F::from(n)
        * factors
            .iter()
            .map(|&x| F::from(BigInt::from(1)) - F::new(BigInt::from(1), BigInt::from(x)))
            .product::<F>()
}

println!(
    "{:?}",
    (1..=1_000_000)
        .map(|i| euler_totient(i).to_usize().unwrap())
        .sum::<usize>()
        - 1
);

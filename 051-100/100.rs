#!/usr/bin/env rust-script

// cargo-deps: rust-bigint = "1.2.0"

// https://oeis.org/A011900
// a(n) = 6*a(n-1) - a(n-2) - 2 with a(0) = 1, a(1) = 3.

use rust_bigint::BigInt;

fn a() -> impl Iterator<Item = BigInt> {
    let mut a = BigInt::one();
    let mut b = BigInt::from(3);
    std::iter::from_fn(move || {
        let c = 6u64 * &b - &a - 2;
        a = b.clone();
        b = c.into();
        Some(a.clone())
    })
}

fn total(blue: BigInt) -> BigInt {
    ((8u64 * blue.pow(2) - 8u64 * blue + 1u64).sqrt() + 1u64) / 2u64
}

println!(
    "{}",
    a().take(100)
        .map(|n| (n.clone(), total(n)))
        .skip_while(|(_, t)| t < &BigInt::from(10).pow(12))
        .next()
        .unwrap()
        .0
);

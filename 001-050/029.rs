#!/usr/bin/env rust-script

// cargo-deps: rust-bigint = "1.2.0", itertools = "0.10.1"

println!(
    "{:?}",
    itertools::iproduct!(2..101, 2..101)
        .map(|(a, b)| rust_bigint::BigInt::from(a).pow(b))
        .collect::<std::collections::HashSet<_>>()
        .len()
);

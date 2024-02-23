#!/usr/bin/env rust-script

// cargo-deps: itertools="0.10.1"

fn pentagonal(n: usize) -> Vec<usize> {
    (1..=n).map(|x| x * (3 * x - 1) / 2).collect()
}

fn is_pentagonal(n: usize) -> bool {
    let x = (1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0;
    x == x.round()
}

println!(
    "{:?}",
    itertools::iproduct!(pentagonal(10000), pentagonal(10000))
        .filter(|&(x, y)| is_pentagonal(x + y) && is_pentagonal(x - y) && x != y)
        .map(|(x, y)| x - y)
        .next()
        .unwrap()
);

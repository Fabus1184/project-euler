#!/usr/bin/env rust-script

// cargo-deps: itertools="0.10.1"

fn triangles() -> impl Iterator<Item = usize> {
    (1..).map(|x| x * (x + 1) / 2)
}

fn is_pentagonal(n: usize) -> bool {
    let x = (1.0 + (1.0 + 24.0 * n as f64).sqrt()) / 6.0;
    x == x.round()
}

fn is_hexagonal(n: usize) -> bool {
    let x = (1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 4.0;
    x == x.round()
}

println!(
    "{:?}",
    triangles()
        .filter(|&x| is_pentagonal(x) && is_hexagonal(x))
        .nth(2)
        .unwrap()
);

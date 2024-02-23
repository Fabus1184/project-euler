#!/usr/bin/env rust-script

fn collatz(mut n: usize) -> Vec<usize> {
    let mut v = vec![n];
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        v.push(n);
    }
    v
}

println!(
    "{}",
    (1..1_000_000)
        .map(|n| (n, collatz(n).len()))
        .max_by_key(|&(_, l)| l)
        .unwrap()
        .0
);

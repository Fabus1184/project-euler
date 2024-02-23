#!/usr/bin/env rust-script

use std::collections::HashMap;

fn square_sum(mut n: i64) -> i64 {
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        sum += d * d;
        n /= 10;
    }
    sum
}

fn chain(n: i64) -> bool {
    match n {
        89 => true,
        1 => false,
        _ => chain(square_sum(n)),
    }
}

println!(
    "{}",
    (1..=10_000_000)
        .filter(|&n| chain(n))
        .count()
);

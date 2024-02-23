#!/usr/bin/env rust-script

// cargo-deps: primal = "0.2.3", itertools = "0.10.1"

use itertools::Itertools;

println!(
    "{:?}",
    (1..10)
        .flat_map(|i| (1..i).permutations(i - 1).collect::<Vec<_>>())
        .map(|v| v.iter().fold(0, |acc, &x| acc * 10 + x))
        .filter(|&x| primal::is_prime(x.try_into().unwrap()))
        .last()
        .unwrap()
);

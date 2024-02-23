#!/usr/bin/env rust-script

// cargo-deps: itertools="0.10.1"

use itertools::Itertools;

println!(
    "{}",
    (0..=9)
        .permutations(10)
        .filter(|p| p[1..]
            .windows(3)
            .map(|c| c.iter().join("").parse::<u32>().unwrap())
            .zip([2, 3, 5, 7, 11, 13, 17].iter())
            .all(|(a, b)| a % b == 0))
        .map(|p| p.iter().join("").parse::<u64>().unwrap())
        .sum::<u64>()
);

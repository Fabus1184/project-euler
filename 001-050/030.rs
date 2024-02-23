#!/usr/bin/env rust-script

println!(
    "{:?}",
    (2..1_000_000)
        .filter(|n| n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().pow(5))
            .sum::<u32>()
            == *n as u32)
        .sum::<u32>()
);

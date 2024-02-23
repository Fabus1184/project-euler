#!/usr/bin/env rust-script

println!(
    "{:?}",
    (1..101).sum::<i64>().pow(2) - (1..101).map(|x| x * x).sum::<i64>()
);

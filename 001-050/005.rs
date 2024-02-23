#!/usr/bin/env rust-script

println!(
    "{:?}",
    (1..)
        .filter(|&n| (1..20).all(|x| n % x == 0))
        .next()
        .unwrap()
);

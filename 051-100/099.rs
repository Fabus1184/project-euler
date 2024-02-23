#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4", rust-bigint = "1.2.0"

use rust_bigint::BigInt;
use std::io::Read;

println!(
    "{}",
    file_fetcher::open_str("https://projecteuler.net/project/resources/p099_base_exp.txt")
        .unwrap()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .collect::<String>()
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let base = BigInt::from_str_radix(parts.next().unwrap(), 10).unwrap();
            let exp = parts.next().unwrap().parse::<u32>().unwrap();
            base.pow(exp)
        })
        .enumerate()
        .max_by_key(|(_, n)| n.clone())
        .unwrap()
        .0
        + 1
);

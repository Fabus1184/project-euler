#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;
use std::collections::{HashMap, HashSet};

let words = file_fetcher::open_str("https://projecteuler.net/project/resources/p042_words.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .split(",")
    .map(|x| x[1..x.len() - 1].to_string())
    .collect::<Vec<_>>();

let mut values: HashMap<u32, u32> = HashMap::new();

for word in words {
    let sum = word.chars().map(|c| c as u32 - 'A' as u32 + 1).sum::<u32>();
    *values.entry(sum).or_insert(0) += 1;
}


let triangle_numbers: HashSet<u32> = (1..)
    .map(|n| n * (n + 1) / 2)
    .take_while(|&n| n <= *values.keys().max().unwrap())
    .collect();

println!(
    "{}",
    values
        .iter()
        .filter(|(&v, _)| triangle_numbers.contains(&v))
        .map(|(_, &c)| c)
        .sum::<u32>()
);

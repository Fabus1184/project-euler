#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4", itertools = "0.10.5"

use std::io::Read;
use itertools::Itertools;
use std::collections::HashSet;

let sets = file_fetcher::open_str("https://projecteuler.net/project/resources/p105_sets.txt")
        .unwrap()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .collect::<String>()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

fn is_special_sum_set(set: &Vec<i32>) -> bool {
    // S(B) ≠ S(C); that is, sums of subsets cannot be equal.
    // If B contains more elements than C then S(B) > S(C).
    let ss = (1..set.len())
        .flat_map(|n| {
            set.iter()
                .combinations(n)
                .map(|v| v.into_iter().map(|&x| x).collect::<HashSet<_>>())
        })
        .collect::<Vec<_>>();

    itertools::iproduct!(ss.iter(), ss.iter())
        .filter(|&(a, b)| a.is_disjoint(b))
        .all(|(a, b)| {
            if a.len() > b.len() {
                a.iter().sum::<i32>() > b.iter().sum::<i32>()
            } else if a.len() < b.len() {
                a.iter().sum::<i32>() < b.iter().sum::<i32>()
            } else {
                a.iter().sum::<i32>() != b.iter().sum::<i32>()
            }
        })
}

println!(
    "{}",
    sets.iter()
        .filter(|set| is_special_sum_set(set))
        .map(|set| set.iter().sum::<i32>())
        .sum::<i32>()
);
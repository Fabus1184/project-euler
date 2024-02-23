#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;
use std::collections::HashSet;

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

// near_optimal = {20, 31, 37, 40, 42, 43, 44}

println!(
    "{}",
    itertools::iproduct!(
        (18..=22),
        (29..=33),
        (35..=39),
        (38..=42),
        (40..=44),
        (41..=45),
        (42..=46)
    )
    .map(|(a, b, c, d, e, f, g)| vec![a, b, c, d, e, f, g])
    .filter(|v| v.iter().unique().count() == 7)
    .filter(is_special_sum_set)
    .min_by_key(|v| v.iter().sum::<i32>())
    .unwrap()
    .iter()
    .join("")
);

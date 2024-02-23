#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;
use std::collections::HashSet;

fn subset_pairs(set: &HashSet<i32>) -> impl Iterator<Item = (HashSet<i32>, HashSet<i32>)> {
    set.iter().combinations(2).map(|v| {
        (
            v[0].iter().collect::<HashSet<_>>(),
            v[1].iter().collect::<HashSet<_>>(),
        )
    })
}

fn main() {
    let v = subset_pairs(&(1..=4).collect::<HashSet<_>>())
        .filter(|(a, b)| a.is_disjoint(&b))
        .collect::<Vec<_>>();

    println!("{v:?}");
}

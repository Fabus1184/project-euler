#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;
use std::collections::HashSet;

fn valid(mut a: Vec<u8>, mut b: Vec<u8>) -> bool {
    if a.contains(&6) {
        a.push(9);
    } else if a.contains(&9) {
        a.push(6);
    }

    if b.contains(&6) {
        b.push(9);
    } else if b.contains(&9) {
        b.push(6);
    }

    let quads = [(0,1), (0,4), (0,9), (1,6), (2,5), (3,6), (4,9), (6,4), (8,1)];
    quads.iter().all(|&(i, j)| {
        a.contains(&i) && b.contains(&j) || a.contains(&j) && b.contains(&i)
    })
}

println!("{:?}", (0..=9)
    .combinations(6)
    .flat_map(|a| (0..=9).combinations(6).map(move |b| (a.clone(), b)))
    .filter(|(a, b)| valid(a.clone(), b.clone()))
    .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
    .collect::<HashSet<_>>()
    .len()
);

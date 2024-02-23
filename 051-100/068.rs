#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;

const GS5: [[u32; 3]; 5] = [[0, 2, 4], [1, 4, 7], [8, 7, 6], [9, 6, 3], [5, 3, 2]];

fn five_gonize(ns: &Vec<u32>) -> [[u32; 3]; 5] {
    let mut gs = [[0; 3]; 5];
    for (i, g) in GS5.iter().enumerate() {
        for (j, &k) in g.iter().enumerate() {
            gs[i][j] = ns[k as usize];
        }
    }
    gs
}

fn is_magic(ns: &[[u32; 3]]) -> bool {
    ns.iter().map(|g| g.iter().sum::<u32>()).all_equal()
}

println!(
    "{}",
    (1..=10)
        .permutations(10)
        .map(|p| five_gonize(&p))
        .filter(|p| is_magic(p))
        .map(|mut p| {
            let m = p.iter().min_by_key(|g| g[0]).unwrap()[0];
            let i = p.iter().position(|g| g[0] == m).unwrap();
            p.rotate_left(i);
            p
        })
        .map(|p| p.map(|g| g.iter().join("")).join(""))
        .sorted()
        .dedup()
        .last()
        .unwrap()
);

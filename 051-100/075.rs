#!/usr/bin/env rust-script

use std::collections::{HashMap, HashSet};

type Triple<T> = (T, T, T);

fn triple_children((a, b, c): Triple<i64>) -> [Triple<i64>; 3] {
    [
        (a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c),
        (a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c),
        (
            -a + 2 * b + 2 * c,
            -2 * a + b + 2 * c,
            -2 * a + 2 * b + 3 * c,
        ),
    ]
}

let mut counts: HashMap<i64, i64> = HashMap::new();
let mut new: HashSet<Triple<i64>> = HashSet::new();
loop {
    new = new
        .clone()
        .iter()
        .flat_map(|&t| triple_children(t))
        .chain(
            if counts.len() == 0 {
                vec![(3, 4, 5)]
            } else {
                vec![]
            },
        )
        .filter(|&(a, b, c)| a + b + c <= 1_500_000)
        .collect();

    for (a, b, c) in new.iter() {
        for i in 1.. {
            let (a, b, c) = (a * i, b * i, c * i);
            if a + b + c > 1_500_000 {
                break;
            }
            *counts.entry(a + b + c).or_insert(0) += 1;
        }
    }

    if new.len() == 0 {
        break;
    }
}

println!("{:?}", counts.values().filter(|&&v| v == 1).count());

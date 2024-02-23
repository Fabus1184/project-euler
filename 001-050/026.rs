#!/usr/bin/env rust-script

use std::collections::HashMap;

struct DecimalFractionPlaces {
    decimals: Vec<i8>,
    recurring: Option<Vec<i8>>,
}

fn decimal_fraction_places(n: i64, d: i64) -> DecimalFractionPlaces {
    let mut decimals = Vec::new();
    let mut recurring = None;
    let mut seen = HashMap::new();
    let mut n = n;
    while n != 0 {
        if seen.contains_key(&n) {
            recurring = Some(decimals.split_off(seen[&n]));
            break;
        }
        seen.insert(n, decimals.len());
        n *= 10;
        decimals.push((n / d) as i8);
        n %= d;
    }
    DecimalFractionPlaces {
        decimals,
        recurring,
    }
}

println!(
    "{:?}",
    (1..1000)
        .map(|d| (d, decimal_fraction_places(1, d)))
        .max_by_key(|(_, x)| x.recurring.as_ref().map_or(0, |x| x.len()))
        .unwrap()
        .0
);

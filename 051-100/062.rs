#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;
use std::collections::HashMap;

let mut map: HashMap<[u8; 10], Vec<u64>> = HashMap::new();

for a in 1.. {
    let x = format!("{}{}", "0123456789", (a as u64).pow(3).to_string());
    let key = x.chars().counts().iter().sorted().map(|(_, &v)| (v - 1) as u8).collect::<Vec<_>>().try_into().unwrap();
    let entry = map.entry(key).or_insert_with(Vec::new);
    entry.push((a as u64).pow(3));

    if entry.len() == 5 {
        println!("{:?}", entry.iter().min().unwrap());
        break;
    }
}

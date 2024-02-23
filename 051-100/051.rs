#!/usr/bin/env rust-script

// cargo-deps: primal = "0.2.3", itertools = "0.10.5"

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn group_chars(n: usize) -> Vec<String> {
    let mut groups: Vec<String> = Vec::new();
    for c in '0'..='9' {
        let is = n.to_string().chars().enumerate().filter(|(_, x)| *x == c).map(|(i, _)| i as u32).collect::<Vec<_>>();
        if is.len() > 1 {
            for i in 1..=is.len() {
                for x in is.iter().permutations(i) {
                    let mut s = n.to_string();
                    for j in x {
                        s.replace_range(*j as usize..*j as usize + 1, "*");
                    }
                    groups.push(s);
                }
            }
        }
    }
    groups
}

let mut map: HashMap<String, HashSet<usize>> = HashMap::new();

for p in primal::Primes::all().skip_while(|&x| x < 10) {
    for x in group_chars(p) {
        let e = map.entry(x).or_insert(HashSet::new());
        e.insert(p);

        if e.len() == 8 {
            println!("{}", e.iter().min().unwrap());
            return Ok(());
        }
    }
}

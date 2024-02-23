#!/usr/bin/env rust-script

use std::collections::{HashMap, HashSet};

fn factorial(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => (2..=n).product(),
    }
}

fn digit_factorial_sum(mut n: u64, memo: &mut HashMap<u64, u64>) -> usize {
    let mut set = HashSet::from([n]);

    loop {
        let x = memo.get(&n).map(|&x| x).unwrap_or(n
                .to_string()
                .chars()
                .map(|c| factorial(c.to_digit(10).unwrap() as u64))
                .sum::<u64>()
            );
        memo.insert(n, x);
        
        if set.insert(x) == false {
            break;
        }

        n = x;
    }

    set.len()
}

let mut memo = HashMap::new();

println!("{:?}", 
    (0..1_000_000)
        .filter(|&i| digit_factorial_sum(i, &mut memo) == 60)
        .count()
);

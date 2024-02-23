#!/usr/bin/env rust-script

// cargo-deps: primal="0.3.2"

use primal::Primes;

fn is_permutation(a: usize, b: usize) -> bool {
    let mut a = a.to_string().chars().collect::<Vec<char>>();
    let mut b = b.to_string().chars().collect::<Vec<char>>();
    a.sort();
    b.sort();
    a == b
}

fn main() {
    let mut i = 1000;
    let mut count = 0;

    let primes = Primes::all()
        .skip_while(|&x| x < 1000)
        .take_while(|&x| x < 10000)
        .filter(|&x| x != 1487)
        .collect::<Vec<usize>>();

    loop {
        if primes.contains(&(i as usize))
            && primes.contains(&((i + 3330) as usize))
            && primes.contains(&((i + 6660) as usize))
        {
            if is_permutation(i, i + 3330) && is_permutation(i, i + 6660) {
                println!(
                    "{}",
                    i.to_string() + &((i + 3330).to_string()) + &((i + 6660).to_string())
                );
                break;
            }
        }
        i += 1;
    }
}

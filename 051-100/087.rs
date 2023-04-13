// cargo-deps: primal = "0.3.2"

use std::collections::HashSet;

let max = 50_000_000;
let mut set = HashSet::new();

for p in primal::Primes::all().map(|p| p * p).take_while(|&p| p < max) {
    for q in primal::Primes::all().map(|q| q * q * q).take_while(|&q| p + q < max) {
        for r in primal::Primes::all().map(|r| r * r * r * r).take_while(|&r| p + q + r < max) {
            set.insert(p + q + r);
        }
    }
}

println!("{}", set.len());
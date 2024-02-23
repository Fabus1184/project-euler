#!/usr/bin/env rust-script

// cargo-deps: primal = "0.2.3"

use primal::Primes;

println!("{:?}", Primes::all().nth(10_000).unwrap());

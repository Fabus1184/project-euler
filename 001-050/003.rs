#!/usr/bin/env rust-script

fn prime_factors(n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

println!("{:?}", prime_factors(600851475143).last().unwrap());

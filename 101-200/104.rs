#!/usr/bin/env rust-script

// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

fn is_valid(n: &BigInt) -> bool {
    let mut s = (n % BigInt::from(10).pow(9)).to_string().chars().collect::<Vec<_>>();
    s.sort();

    if s != vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
        return false;
    }

    let mut s = n.to_string().chars().take(9).collect::<Vec<_>>();
    s.sort();

    s == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

let mut a = BigInt::from(1);
let mut b = BigInt::from(1);
let mut i = 2;
while !is_valid(&b) {
    let c = a + b.clone();
    a = b;
    b = c;
    i += 1;
}
println!("{}", i);
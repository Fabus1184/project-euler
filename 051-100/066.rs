// cargo-deps: rust-bigint="1.2.0"

use rust_bigint::BigInt;

fn sqrt_convergents(n: u64) -> Vec<(BigInt, BigInt)> {
    let a = (n as f64).sqrt() as u64;
    let mut vec = vec![(BigInt::from(a), BigInt::from(1))];
    for _ in 1..100 {
        vec.push((
            (BigInt::from(n) - vec.last().unwrap().1.pow(2)) / vec.last().unwrap().0,
            (BigInt::from(n) - vec.last().unwrap().1.pow(2)) / vec.last().unwrap().0,
        ));
    }
    vec
}

println!("{:?}", sqrt_convergents(2));

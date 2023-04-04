// cargo-deps: rust-bigint = "1.2.0", itertools = "0.10.5"

use rust_bigint::BigInt;

println!("{}", 
    itertools::iproduct!(1..100, 1..100)
        .map(|(a, b)| BigInt::from(a).pow(b))
        .map(|n| n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
        .max()
        .unwrap()
)
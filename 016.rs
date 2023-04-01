// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

println!(
    "{}",
    BigInt::from(2)
        .pow(1000)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
);

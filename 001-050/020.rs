// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

println!(
    "{}",
    (1..=100)
        .map(|x| BigInt::from(x))
        .fold(BigInt::from(1), |acc, x| acc * x)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum::<u32>()
);

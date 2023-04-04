// cargo-deps: rust-bigint = "1.2.0"

fn series() -> impl Iterator<Item = rust_bigint::BigInt> {
    (1..=1000).map(|x| rust_bigint::BigInt::from(x).pow(x))
}

println!(
    "{}",
    series()
        .fold(rust_bigint::BigInt::from(0), |acc, x| acc + x)
        .to_string()
        .chars()
        .collect::<Vec<char>>()
        .windows(10)
        .last()
        .unwrap()
        .iter()
        .collect::<String>()
);

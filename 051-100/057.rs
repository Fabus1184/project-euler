// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

fn sqrt2_expansion(n: u64) -> (BigInt, BigInt) {
    match n {
        0 => (BigInt::from(1), BigInt::from(2)),
        1 => (BigInt::from(3), BigInt::from(2)),
        _ => {
            let (a, b) = sqrt2_expansion(n - 1);
            (a.clone() + BigInt::from(2) * b.clone(), a + b)
        }
    }
}

println!(
    "{}",
    (1..=1000)
        .map(|n| sqrt2_expansion(n))
        .filter(|(a, b)| a.to_string().len() > b.to_string().len())
        .count()
);

// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

println!(
    "{}",
    (28433u64 * BigInt::from(2).pow(7830457) + 1) % 10u64.pow(10)
);

// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

fn ncr(n: u64, r: u64) -> BigInt {
    let mut res = BigInt::from(1);
    for i in 1..=r {
        res *= BigInt::from(n - i + 1);
        res /= BigInt::from(i);
    }
    res
}

fn main() {
    let mut res = 0;
    for n in 1..=100 {
        for k in 1..n {
            if ncr(n, k) > BigInt::from(1_000_000) {
                res += 1;
            }
        }
    }
    println!("{}", res);
}

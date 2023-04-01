// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

struct Fibonacci {
    a: BigInt,
    b: BigInt,
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        a: BigInt::from(0),
        b: BigInt::from(1),
    }
}

impl Iterator for Fibonacci {
    type Item = BigInt;

    fn next(&mut self) -> Option<BigInt> {
        let x = self.a.clone();
        self.a += self.b.clone();
        self.b = x.clone();
        Some(x)
    }
}

println!(
    "{:?}",
    fibonacci()
        .enumerate()
        .find(|(_, x)| x.to_str_radix(10).len() >= 1000)
        .unwrap()
        .0
);

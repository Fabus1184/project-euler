// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;
use std::collections::HashMap;

fn p(n: usize, memo: &mut HashMap<usize, BigInt>) -> BigInt {
    if n == 0 {
        return BigInt::from(1);
    }

    if let Some(x) = memo.get(&n) {
        return x.clone();
    }

    let mut sum = BigInt::from(0);
    let mut k = 1;
    loop {
        let x = k * (3 * k - 1) / 2;
        if x > n {
            break;
        }
        let y = p(n - x, memo);
        if k % 2 == 1 {
            sum += y;
        } else {
            sum -= y;
        }
        k += 1;
    }
    k = 1;
    loop {
        let x = k * (3 * k + 1) / 2;
        if x > n {
            break;
        }
        let y = p(n - x, memo);
        if k % 2 == 1 {
            sum += y;
        } else {
            sum -= y;
        }
        k += 1;
    }
    memo.insert(n, sum.clone());
    sum
}

let mut memo = HashMap::new();
println!("{}", (1..)
    .skip_while(|&x| p(x, &mut memo) % 1_000_000 != BigInt::from(0))
    .next()
    .unwrap()
);

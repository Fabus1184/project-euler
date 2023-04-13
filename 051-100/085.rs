// cargo-deps: itertools = "0.10.5"

use std::collections::HashMap;

fn rectangle_count(m: u64, n: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    match (m, n) {
        (0, _) | (_, 0) => 0,
        (1, _) => n,
        (_, 1) => m,
        _ => itertools::iproduct!(1..=m, 1..=n)
            .filter(|&x| x != (m, n))
            .map(|(x, y)| rectangle_count(x, y, memo) + rectangle_count(y, x, memo))
            .sum::<u64>()
    }
}

let mut memo = HashMap::new();
let mut min = (u64::MAX, 0, 0);
for m in 1..=30 {
    for n in 1..=m {
        let count = rectangle_count(m, n, &mut memo) / 2;
        let diff = (2_000_000i128 - count as i128).abs();
        if diff < (2_000_000i128 - min.0 as i128).abs() {
            min = (count, m, n);
        }

        if count > 2_000_000 {
            break;
        }
    }
}
println!("{:?} {}", min, (2_000_000i128 - min.0 as i128).abs());

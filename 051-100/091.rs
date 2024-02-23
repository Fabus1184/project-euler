#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;

let max = 50;
let mut count = 0;

for (x1, y1) in itertools::iproduct!(0i32..=max, 0..=max) {
    for (x2, y2) in itertools::iproduct!(0..=max, 0..=max) {
        if x1 == x2 && y1 == y2 || x1 == 0 && y1 == 0 || x2 == 0 && y2 == 0 {
            continue;
        }

        let mut ds = [
            (x1 - x2).pow(2) + (y1 - y2).pow(2),
            (x1.pow(2) + y1.pow(2)),
            (x2.pow(2) + y2.pow(2)),
        ];
        ds.sort();

        let [a, b, c] = ds;

        if a + b == c {
            count += 1;
        }
    }
}

println!("{}", count / 2);

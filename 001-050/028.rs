#!/usr/bin/env rust-script

fn sum_of_diagonals(n: usize) -> usize {
    let mut sum = 1;
    let mut x = 1;
    for i in 1..n / 2 + 1 {
        for _ in 0..4 {
            x += 2 * i;
            sum += x;
        }
    }
    sum
}

println!("{:?}", sum_of_diagonals(1001));

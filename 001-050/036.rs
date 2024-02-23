#!/usr/bin/env rust-script

fn is_palindrome(n: &str) -> bool {
    n == n.chars().rev().collect::<String>()
}

println!(
    "{}",
    (0..1_000_000)
        .filter(|&n| is_palindrome(&n.to_string()) && is_palindrome(&format!("{:b}", n)))
        .sum::<u64>()
)

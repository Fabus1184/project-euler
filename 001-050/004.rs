#!/usr/bin/env rust-script

fn is_palindrome(n: usize) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

println!(
    "{:?}",
    (100..999)
        .flat_map(|x| (x..999).map(move |y| x * y))
        .filter(|&x| is_palindrome(x))
        .max()
        .unwrap()
);

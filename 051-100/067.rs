#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;

let triangle = file_fetcher::open_str("https://projecteuler.net/project/resources/p067_triangle.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .lines()
    .map(|line| {
        line.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

fn max_path_sum(triangle: &Vec<Vec<u32>>) -> u32 {
    let mut dp = vec![vec![0; triangle.len()]; triangle.len()];
    dp[0][0] = triangle[0][0];
    for i in 1..triangle.len() {
        for j in 0..triangle[i].len() {
            if j == 0 {
                dp[i][j] = dp[i - 1][j] + triangle[i][j];
            } else if j == triangle[i].len() - 1 {
                dp[i][j] = dp[i - 1][j - 1] + triangle[i][j];
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j - 1], dp[i - 1][j]) + triangle[i][j];
            }
        }
    }
    *dp[triangle.len() - 1].iter().max().unwrap()
}

println!("{:?}", max_path_sum(&triangle));

#!/usr/bin/env rust-script

fn coin_sums(n: usize, atoms: &Vec<usize>) -> usize {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for &atom in atoms {
        for i in atom..=n {
            dp[i] += dp[i - atom];
        }
    }
    dp[n]
}

println!("{}", coin_sums(100, &Vec::from_iter(1..=99)));

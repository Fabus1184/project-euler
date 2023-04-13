// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;

let matrix = file_fetcher::open_str("https://projecteuler.net/project/resources/p081_matrix.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .lines()
    .map(|line| {
        line.split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
dp[0][0] = matrix[0][0];

for i in 1..matrix.len() {
    dp[i][0] = dp[i - 1][0] + matrix[i][0];
}

for j in 1..matrix[0].len() {
    dp[0][j] = dp[0][j - 1] + matrix[0][j];
}

for i in 1..matrix.len() {
    for j in 1..matrix[0].len() {
        dp[i][j] = matrix[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
    }
}

println!("{}", dp[matrix.len() - 1][matrix[0].len() - 1]);
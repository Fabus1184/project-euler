fn binom(n: u64, k: u64) -> u64 {
    (1..=k).fold(1, |acc, i| acc * (n - i + 1) / i)
}

fn rectangle_count(m: u64, n: u64) -> u64 {
    binom(m + 1, 2) * binom(n + 1, 2)
}

println!(
    "{:?}",
    (1..=100)
        .flat_map(|i| (1..=i).map(move |j| (i, j)))
        .min_by_key(|&(i, j)| (2_000_000 - rectangle_count(i, j) as i64).abs())
        .map(|(i, j)| i * j)
        .unwrap()
);

fn diagonal_sums(n: u64) -> u64 {
    (0..(n + 1) / 2)
        .map(|n| {
            let a = (2 * n + 1).pow(2);
            let b = (2 * n + 1).pow(2) - 2 * n;
            let c = (2 * n + 1).pow(2) - 4 * n;
            let d = (2 * n + 1).pow(2) - 6 * n;

            a + b + c + d
        })
        .sum::<u64>()
        - 3
}

pub fn solution() -> u64 {
    diagonal_sums(1001)
}

pub fn collatz_length(n: u64) -> usize {
    std::iter::successors(Some(n), |&x| match x {
        1 => None,
        x if x % 2 == 0 => Some(x / 2),
        x => Some(3 * x + 1),
    })
    .count()
}

pub fn solution() -> u64 {
    (1..1_000_000).max_by_key(|&x| collatz_length(x)).unwrap()
}

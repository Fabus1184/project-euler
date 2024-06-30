use itertools::Itertools;

pub fn solution() -> u64 {
    (0..=9)
        .permutations(10)
        .nth(999_999)
        .unwrap()
        .iter()
        .fold(0, |acc, &n| acc * 10 + n as u64)
}

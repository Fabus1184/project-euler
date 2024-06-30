pub fn solution() -> u64 {
    let b = (2..=100).fold(bnum::types::U1024::ONE, |acc: bnum::types::U1024, x| {
        acc * bnum::types::U1024::from_digit(x)
    });
    b.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .sum()
}

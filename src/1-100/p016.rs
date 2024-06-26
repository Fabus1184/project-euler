pub fn solution() -> u32 {
    bnum::types::U1024::from_digit(2)
        .pow(1000)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

pub fn solution() -> u32 {
    let sum = (1..=100).sum::<u32>();
    let sum_of_squares = (1..=100).map(|x| x * x).sum::<u32>();
    sum * sum - sum_of_squares
}

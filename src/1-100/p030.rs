use crate::lib::prelude::Arithmetic;

fn is_sum_of_fifth_powers_of_digits(n: u32) -> bool {
    n.digits().iter().map(|&d| (d as u32).pow(5)).sum::<u32>() == n
}

pub fn solution() -> u32 {
    (2..1_000_000)
        .filter(|&n| is_sum_of_fifth_powers_of_digits(n))
        .sum()
}

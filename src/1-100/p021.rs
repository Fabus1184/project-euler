use crate::lib::prelude::Arithmetic;

fn amicables() -> impl Iterator<Item = u32> {
    (2_u32..).filter(|&x| {
        let divisor_sum = x.proper_divisors().sum::<u32>();
        divisor_sum != x && divisor_sum.proper_divisors().sum::<u32>() == x
    })
}

pub fn solution() -> u32 {
    amicables().take_while(|&x| x < 10_000).sum()
}

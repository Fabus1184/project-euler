use crate::traits::NumberThings;

pub fn solution() -> u32 {
    (1..=20).fold(1, |acc, x| acc.lcm(x))
}

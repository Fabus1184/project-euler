use crate::lib::prelude::*;

pub fn solution() -> u32 {
    (1..).filter(|x| x.is_prime()).nth(10000).unwrap()
}

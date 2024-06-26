use crate::lib::prelude::*;

pub fn solution() -> u64 {
    u64::primes().take_while(|&x| x < 2_000_000).sum()
}

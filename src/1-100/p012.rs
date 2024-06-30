use crate::lib::prelude::Arithmetic;

fn triangle_numbers() -> impl Iterator<Item = u32> {
    (1..).scan(0, |state, x| {
        *state += x;
        Some(*state)
    })
}

pub fn solution() -> u32 {
    triangle_numbers()
        .find(|&n| n.proper_divisors().count() > 500)
        .unwrap()
}

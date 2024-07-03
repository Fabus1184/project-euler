use itertools::Itertools;

use crate::lib::prelude::Arithmetic;

pub fn solution() -> usize {
    itertools::iproduct!(2_u32..=100, 2_u32..=100)
        .map(|(a, b)| {
            std::iter::repeat(a.prime_factors())
                .take(b as usize)
                .flatten()
                .sorted()
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
}

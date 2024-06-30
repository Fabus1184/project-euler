use itertools::Itertools;

fn fibonnaci() -> impl Iterator<Item = bnum::BUint<64>> {
    std::iter::successors(Some((bnum::BUint::ONE, bnum::BUint::ZERO)), |&(a, b)| {
        Some((b, a + b))
    })
    .map(|(_, b)| b)
}

pub fn solution() -> usize {
    fibonnaci()
        .find_position(|&n| n.to_string().len() == 1000)
        .unwrap()
        .0
}

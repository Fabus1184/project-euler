/// generate all pythagorean triplets using an infinite ternary tree
fn pythagorean_triplets() -> impl Iterator<Item = (u64, u64, u64)> {
    (1..).flat_map(|m| {
        (1..m).map(move |n| {
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            (a, b, c)
        })
    })
}

pub fn solution() -> u64 {
    let (a, b, c) = pythagorean_triplets()
        .find(|(a, b, c)| a + b + c == 1000)
        .unwrap();

    a * b * c
}

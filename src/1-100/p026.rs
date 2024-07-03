// infinite iterator over decimals of 1/n
fn decimals(n: u32) -> impl Iterator<Item = u8> {
    let mut remainder = 1;
    std::iter::from_fn(move || {
        remainder *= 10;
        let quotient = remainder / n;
        remainder %= n;
        Some(quotient as u8)
    })
}

fn cycle_length<T: PartialEq + Copy + std::fmt::Debug>(iter: impl Iterator<Item = T>) -> usize {
    let mut agg = Vec::new();

    for item in iter {
        agg.push(item);

        if agg.len() > 10 {
            for i in 0..agg.len() / 2 {
                let (a, b) = agg[i..].split_at((agg.len() - i) / 2);
                if a == b {
                    return a.len();
                }
            }
        }
    }

    0
}

pub fn solution() -> u32 {
    (2..1000)
        .max_by_key(|&x| cycle_length(decimals(x)))
        .unwrap()
}

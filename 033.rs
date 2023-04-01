// cargo-deps: itertools="0.10.1"

use itertools::Itertools;

fn cancel_digits(a: u64, b: u64) -> Option<((u64, u64), (u64, u64))> {
    let a_digits = a.to_string().chars().collect::<String>();
    let b_digits = b.to_string().chars().collect::<String>();

    for (ad, bd) in itertools::iproduct!(0..a_digits.len(), 0..b_digits.len()) {
        let mut num = a_digits.clone();
        let mut den = b_digits.clone();

        if num.remove(ad) != den.remove(bd) {
            continue;
        }

        if let (Ok(x), Ok(y)) = (num.parse::<u64>(), den.parse::<u64>()) {
            if (a as f64) / (b as f64) == (x as f64) / (y as f64) {
                return Some(((a, b), (x, y)));
            }
        }
    }

    None
}

fn lowest_common_terms((mut a, mut b): (u64, u64)) -> (u64, u64) {
    let mut i = 2;
    while i <= a && i <= b {
        if a % i == 0 && b % i == 0 {
            a /= i;
            b /= i;
        } else {
            i += 1;
        }
    }

    (a, b)
}

println!(
    "{:?}",
    lowest_common_terms(
        itertools::iproduct!(10..100, 10..100)
            .filter(|(a, b)| a % 10 != 0 && b % 10 != 0)
            .filter(|(a, b)| a < b)
            .filter(|(a, b)| a.to_string().chars().dedup().count() == 2
                && b.to_string().chars().dedup().count() == 2)
            .filter_map(|(a, b)| cancel_digits(a, b))
            .map(|(_, x)| x)
            .fold((1, 1), |(a, b), (x, y)| lowest_common_terms((a * x, b * y)))
    )
    .1
);

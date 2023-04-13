// cargo-deps: arpfloat = "0.1.9"

use arpfloat::{Float, Semantics, RoundingMode};
const FP: Semantics = Semantics::new(10, 512, RoundingMode::NearestTiesToEven);


let mut sum = 0;

for i in 1..=100 {
    if [1, 4, 9, 16, 25, 36, 49, 64, 81, 100].contains(&i) {
        continue;
    }

    sum += Float::from_i64(FP, i)
        .sqrt()
        .to_string()
        .chars()
        .filter(|c| c.is_digit(10))
        .take(100)
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
}

println!("{}", sum);

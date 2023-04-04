// cargo-deps: rust-bigint = "1.2.0"

use rust_bigint::BigInt;

fn evaluate_continued_fraction(first: BigInt, terms: &[u32]) -> (BigInt, BigInt) {
    let (mut num, mut den) = (BigInt::from(1), BigInt::from(0));
    for &term in terms.iter().rev() {
        let (new_num, new_den) = (den.clone(), num.clone());
        num = new_num + BigInt::from(term) * new_den.clone();
        den = new_den;
    }
    (first * num.clone() + den, num)
}

fn e_terms() -> impl Iterator<Item=u32> {
    (0..).map(|n| {
        if n % 3 == 1 {
            2 * (n / 3 + 1)
        } else {
            1
        }
    })
}

let (num, _) = evaluate_continued_fraction(BigInt::from(2), &e_terms().take(99).collect::<Vec<_>>());
println!("{}", num.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());

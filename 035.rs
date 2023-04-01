// cargo-deps: primal = "0.2.3"

use primal::Primes;

fn rotations(n: u64) -> Vec<u64> {
    let mut digits = n.to_string().chars().collect::<Vec<_>>();
    let mut rotations = Vec::new();

    for _ in 0..digits.len() {
        rotations.push(
            digits
                .iter()
                .map(|&x| x.to_digit(10).unwrap() as u64)
                .fold(0, |acc, x| acc * 10 + x),
        );
        digits.rotate_left(1);
    }

    rotations
}

println!(
    "{}",
    Primes::all()
        .take_while(|&x| x < 1_000_000)
        .filter(|&x| rotations(x.try_into().unwrap())
            .iter()
            .all(|&x| primal::is_prime(x)))
        .collect::<Vec<_>>()
        .len()
);

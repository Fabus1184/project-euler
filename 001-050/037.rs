// cargo-deps: primal = "0.2.3"

use primal::Primes;

fn is_truncatable_prime(n: usize) -> bool {
    let digits = n.to_string().chars().collect::<Vec<_>>();

    for i in 0..digits.len() {
        if !primal::is_prime(
            digits
                .iter()
                .skip(i)
                .fold(0, |acc, &x| acc * 10 + x.to_digit(10).unwrap() as u64),
        ) || !primal::is_prime(
            digits
                .iter()
                .take(digits.len() - i)
                .fold(0, |acc, &x| acc * 10 + x.to_digit(10).unwrap() as u64),
        ) {
            return false;
        }
    }

    true
}

println!(
    "{}",
    Primes::all()
        .take_while(|&x| x < 1_000_000)
        .skip_while(|&x| x < 10)
        .filter(|&x| is_truncatable_prime(x))
        .sum::<usize>()
);

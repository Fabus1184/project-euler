// cargo-deps: primal = "0.2.3", itertools = "0.10.1"

fn number_of_primes(a: i32, b: i32) -> usize {
    let f = |n| n * n + a * n + b;
    (0..)
        .map(f)
        .take_while(|&n| n >= 0 && primal::is_prime(n as u64))
        .count()
}

println!(
    "{:?}",
    itertools::iproduct!(-999..1000, -999..1000)
        .map(|(a, b)| (a, b, number_of_primes(a, b)))
        .max_by_key(|&(_, _, n)| n)
        .map(|(a, b, _)| a * b)
        .unwrap()
);

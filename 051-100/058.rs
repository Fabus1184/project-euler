// cargo-deps: primal="0.3.2"

fn bottom_right(n: u64) -> u64 {
    return (2 * n + 1).pow(2);
}

fn bottom_left(n: u64) -> u64 {
    return bottom_right(n) - 2 * n;
}

fn top_left(n: u64) -> u64 {
    return bottom_right(n) - 4 * n;
}

fn top_right(n: u64) -> u64 {
    return bottom_right(n) - 6 * n;
}

let mut primes = 0;
let mut diagonals: Vec<[u64; 4]> = Vec::new();
for n in 1.. {
    let new = [bottom_right(n), bottom_left(n), top_left(n), top_right(n)];
    primes += new.iter().filter(|&x| primal::is_prime(*x)).count();
    diagonals.push(new);
    
    let total = diagonals.len() * 4 + 1;

    if primes * 10 < total {
        println!("{}", 2 * n + 1);
        break;
    }
}

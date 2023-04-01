fn distinct_prime_factors(n: u64) -> u64 {
    let mut factors = Vec::new();
    let mut d = 2;
    let mut nn = n;

    while nn > 1 {
        while nn % d == 0 {
            nn /= d;
            factors.push(d);
        }
        d += 1;
        if d * d > nn {
            if nn > 1 {
                factors.push(nn);
            }
            break;
        }
    }

    factors.sort();
    factors.dedup();
    factors.len() as u64
}

let mut i = 1;
let mut count = 0;

loop {
    if distinct_prime_factors(i) == 4 {
        count += 1;
        if count == 4 {
            println!("{}", i - 3);
            break;
        }
    } else {
        count = 0;
    }
    i += 1;
}


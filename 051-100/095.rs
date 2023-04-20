use std::collections::HashSet;

fn sum_of_proper_divisors(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            sum += i;
            if i != 1 && i != n / i {
                sum += n / i;
            }
        }
    }
    sum
}

let mut max = (0, HashSet::new());
for i in 1..1_000_000 {
    let mut x = i;
    let mut set = HashSet::new();

    while set.insert(x) {
        x = sum_of_proper_divisors(x);

        if x > 1_000_000 {
            break;
        }
    }

    if x == i && set.len() > max.0 {
        max = (set.len(), set);
    }
}

println!("{}", max.1.iter().min().unwrap());

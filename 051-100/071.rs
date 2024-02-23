#!/usr/bin/env rust-script

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}


let mut max: (f64, u64, u64) = (0f64, 0, 0);

for i in 1..1_000_000 {
    let (n, d) = (((3f64 / 7f64) * i as f64).floor() as u64, i);
    
    if (n, d) == (3, 7) || gcd(n, d) != 1 {
        continue;
    }

    if (n as f64 / d as f64) > max.0 {
        max = (n as f64 / d as f64, n, d);
    }

}

println!("{}", max.1);

#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.1"

for a in 1.. {
    for b in 1..a {
        for c in 1..b {
            if a * a == b * b + c * c && a + b + c == 1000{
                println!("{}", a * b * c);
                return Ok(());
            }
        }
    }
}

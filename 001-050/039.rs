#!/usr/bin/env rust-script

let mut max = 0;
let mut max_p = 0;
for p in 3..1001 {
    let mut count = 0;
    for a in 1..p {
        for b in a..p {
            let c = p - a - b;
            if a * a + b * b == c * c {
                count += 1;
            }
        }
    }
    if count > max {
        max = count;
        max_p = p;
    }
}
println!("{}", max_p);

#!/usr/bin/env rust-script

let mut count = 0;
for b in 0u64..10 {
    for e in 1u32..25 {
        if b.pow(e).to_string().len() == e.try_into().unwrap() {
            count += 1;
        }
    }
}

println!("{}", count);

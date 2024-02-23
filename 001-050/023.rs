#!/usr/bin/env rust-script

fn abundant_numbers() -> impl Iterator<Item = usize> {
    (1..).filter(|&x| {
        let mut sum = 1;
        let mut i = 2;
        while i * i <= x {
            if x % i == 0 {
                sum += i;
                if i * i != x {
                    sum += x / i;
                }
            }
            i += 1;
        }
        sum > x
    })
}

let abundant_numbers = abundant_numbers()
    .take_while(|&x| x < 28123)
    .collect::<Vec<_>>();
let mut sum = 0;
for i in 1..28123 {
    if !abundant_numbers.iter().any(|&x| {
        abundant_numbers
            .iter()
            .take_while(|&y| y <= &i)
            .any(|&y| x + y == i)
    }) {
        sum += i;
    }
}
println!("{}", sum);

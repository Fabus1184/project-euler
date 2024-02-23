#!/usr/bin/env rust-script

fn lexicographic_permute<T: PartialOrd>(vec: &mut Vec<T>)
{
    let mut i = vec.len() - 1;
    while i > 0 && vec[i - 1] >= vec[i] {
        i -= 1;
    }
    if i == 0 {
        return;
    }
    let mut j = vec.len() - 1;
    while vec[j] <= vec[i - 1] {
        j -= 1;
    }
    vec.swap(i - 1, j);
    vec[i..].reverse();
}

let mut v: Vec<i32> = (0..10).collect();

for _ in 1..1_000_000 {
    lexicographic_permute(&mut v);
}

println!("{}", v.iter().map(|&x| x.to_string()).collect::<String>());

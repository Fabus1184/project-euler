// cargo-deps: file-fetcher = "0.1.4", itertools = "0.10.5"

use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;

let words = file_fetcher::open_str("https://projecteuler.net/project/resources/p098_words.txt")
    .unwrap()
    .bytes()
    .filter_map(|b| b.ok())
    .filter(|&b| b != b'"')
    .fold((Vec::new(), String::new()), |(mut words, mut word), b| {
        if b == b',' {
            words.push(word);
            (words, String::new())
        } else {
            word.push(b as char);
            (words, word)
        }
    })
    .0;

let mut max = 0;
for w1 in words.iter() {
    let anagrams = words
        .iter()
        .filter(|&w2| w1 != w2 && w1.chars().counts() == w2.chars().counts());

    let c1 = w1.chars().next().unwrap();

    for w2 in anagrams {
        if c1 == w2.chars().next().unwrap() {
            for p in (0..10).permutations(w1.chars().unique().count()) {
                let map = w1.chars().unique().zip(p.iter()).collect::<HashMap<_, _>>();

                if *map[&c1] == 0 || *map[&w2.chars().next().unwrap()] == 0 {
                    continue;
                }

                let n1 = w1.chars().map(|c| map[&c]).fold(0, |n, d| n * 10 + d);
                let n2 = w2.chars().map(|c| map[&c]).fold(0, |n, d| n * 10 + d);

                if (n1 as f64).sqrt().fract() == 0.0 && (n2 as f64).sqrt().fract() == 0.0 {
                    if n1 > max {
                        max = n1;
                    }
                    if n2 > max {
                        max = n2;
                    }
                }
            }
        }
    }
}

println!("{}", max);


// cargo-deps: file-fetcher = "0.1.4", itertools = "0.10.5"

use std::io::Read;
use itertools::Itertools;
use std::collections::HashMap;

let cipher = file_fetcher::open_str("https://projecteuler.net/project/resources/p059_cipher.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .split(",")
    .map(|x| x.parse::<u8>().unwrap() as char)
    .collect::<String>();

let english_freq: HashMap<char, f64> = HashMap::from_iter(vec![
    ('a', 0.08167),('b', 0.01492),('c', 0.02782),('d', 0.04253),('e', 0.12702),('f', 0.02228),
    ('g', 0.02015),('h', 0.06094),('i', 0.06966),('j', 0.00153),('k', 0.00772),('l', 0.04025),
    ('m', 0.02406),('n', 0.06749),('o', 0.07507),('p', 0.01929),('q', 0.00095),('r', 0.05987),
    ('s', 0.06327),('t', 0.09056),('u', 0.02758),('v', 0.00978),('w', 0.02360),('x', 0.00150),
    ('y', 0.01974),('z', 0.00074),
]);

let mut scores: HashMap<(char, char, char), f64> = HashMap::new();

for (a, b, c) in itertools::iproduct!('a'..='z', 'a'..='z', 'a'..='z') {
    scores.insert((a, b, c), cipher
        .chars()
        .enumerate()
        .map(|(i, x)| (x as u8) ^ ([a, b, c][i % 3] as u8))
        .map(|x| (x as char).to_ascii_lowercase())
        .counts()
        .into_iter()
        .map(|(k, v)| (k, v as f64 / cipher.len() as f64))
        .map(|(k, v)| english_freq.get(&k).map(|x| x * v).unwrap_or(0.0))
        .sum::<f64>()
    );
}

let key = scores.iter().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap().0;
let decoded = cipher
    .chars()
    .enumerate()
    .map(|(i, x)| (x as u8 ^ [key.0, key.1, key.2][i % 3] as u8) as char)
    .collect::<String>();

println!("{}\n{}", decoded, decoded.chars().map(|x| x as u32).sum::<u32>());
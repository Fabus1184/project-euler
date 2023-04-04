// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;

let mut names: Vec<String> = file_fetcher::open_str("https://projecteuler.net/project/resources/p022_names.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .split(",")
    .map(|x| x[1..x.len() - 1].to_string())
    .collect::<Vec<_>>();

names.sort();

println!("{}", 
    names.iter().enumerate().map(|(i, name)| {
        let score = name.chars().map(|c| (c as u32) - ('A' as u32) + 1).sum::<u32>();
        (i + 1) as u32 * score
    }).sum::<u32>()
);
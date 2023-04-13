// cargo-deps: file-fetcher = "0.1.4", itertools = "0.10.5"

use std::io::Read;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

let keylog = file_fetcher::open_str("https://projecteuler.net/project/resources/p079_keylog.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .map(|line| (line[0].to_digit(10).unwrap(), line[1].to_digit(10).unwrap(), line[2].to_digit(10).unwrap()))
    .collect::<Vec<_>>();

let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
for (a, b, c) in keylog {
    map.entry(a).or_insert(HashSet::new()).insert(b);
    map.entry(b).or_insert(HashSet::new()).insert(c);
}

let mut code: Vec<u32> = vec![*map.values().sorted_by_key(|x| x.len()).next().unwrap().iter().next().unwrap()];

while map.len() > 0 {
    let code_set: HashSet<u32> = code.iter().map(|&x| x).collect();
    let x = *map.iter().sorted_by_key(|(_, v)| v.difference(&code_set).collect::<Vec<_>>().len()).next().unwrap().0;
    code.push(x);
    map.remove(&x);
}

code.reverse();

println!("{}", code.iter().join(""));
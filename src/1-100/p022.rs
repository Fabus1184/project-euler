use itertools::Itertools;
use proc_macros::include_http;

const NAMES: &str = include_http!("https://projecteuler.net/resources/documents/0022_names.txt");

pub fn solution() -> u64 {
    NAMES
        .replace('"', "")
        .split(',')
        .sorted()
        .enumerate()
        .map(|(i, name)| (i + 1) as u64 * name.chars().map(|c| c as u64 - 64).sum::<u64>())
        .sum()
}

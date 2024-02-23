#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4", itertools = "0.10.5"

use std::io::Read;
use std::collections::HashSet;
use itertools::Itertools;

fn solve(field: &mut Vec<u32>) -> bool {
    if let Some(index) = field.iter().position(|&x| x == 0) {
        let row = index / 9;
        let col = index % 9;
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        let possible_values: HashSet<u32> = (1..=9).collect::<HashSet<_>>()
            .difference(&(0..9).map(|i| field[row * 9 + i])
                              .chain((0..9).map(|i| field[i * 9 + col]))
                              .chain((0..9).map(|i| field[(box_row + (i / 3)) * 9 + box_col + (i % 3)]))
                              .filter(|&val| val != 0).collect::<HashSet<_>>())
            .cloned().collect::<HashSet<_>>();
        for value in &possible_values {
            field[index] = *value;
            if solve(field) {
                return true;
            }
        }
        field[index] = 0;
        false
    } else {
        true
    }
}

let mut sudokus = file_fetcher::open_str("https://projecteuler.net/project/resources/p096_sudoku.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .chunks(9 * 10 + 8)
    .into_iter()
    .map(|x| x.skip(7).filter(|&c| c != '\n').map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
    .collect::<Vec<_>>();

let mut sum = 0;
for s in &mut sudokus {
    solve(s);
    sum += s[0] * 100 + s[1] * 10 + s[2];
}

println!("{}", sum);

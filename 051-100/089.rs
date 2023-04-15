// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;

fn parse_roman(roman: &str) -> u32 {
    let mut result = 0;
    let mut last = 0;
    for c in roman.chars().rev() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid character"),
        };
        if value < last {
            result -= value;
        } else {
            result += value;
        }
        last = value;
    }
    result
}

fn to_romans(mut n: u32) -> String {
    let mut result = String::new();
    while n > 0 {
        if n >= 1000 {
            result.push('M');
            n -= 1000;
        } else if n >= 900 {
            result.push_str("CM");
            n -= 900;
        } else if n >= 500 {
            result.push('D');
            n -= 500;
        } else if n >= 400 {
            result.push_str("CD");
            n -= 400;
        } else if n >= 100 {
            result.push('C');
            n -= 100;
        } else if n >= 90 {
            result.push_str("XC");
            n -= 90;
        } else if n >= 50 {
            result.push('L');
            n -= 50;
        } else if n >= 40 {
            result.push_str("XL");
            n -= 40;
        } else if n >= 10 {
            result.push('X');
            n -= 10;
        } else if n >= 9 {
            result.push_str("IX");
            n -= 9;
        } else if n >= 5 {
            result.push('V');
            n -= 5;
        } else if n >= 4 {
            result.push_str("IV");
            n -= 4;
        } else if n >= 1 {
            result.push('I');
            n -= 1;
        }
    }
    result
}

println!("{}", file_fetcher::open_str("https://projecteuler.net/project/resources/p089_roman.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .lines()
    .map(|x| x.len() - to_romans(parse_roman(x)).len())
    .sum::<usize>()
);
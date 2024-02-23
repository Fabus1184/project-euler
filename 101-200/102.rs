#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;

fn contains_origin(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
    let abc = (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1);
    let aoa = -a.0 * c.1 + c.0 * a.1;
    let bob = -b.0 * a.1 + a.0 * b.1;
    let coc = -c.0 * b.1 + b.0 * c.1;
    (aoa.signum() == abc.signum())
        && (bob.signum() == abc.signum())
        && (coc.signum() == abc.signum())
}

println!(
    "{:?}",
    file_fetcher::open_str("https://projecteuler.net/project/resources/p102_triangles.txt")
        .unwrap()
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .collect::<String>()
        .lines()
        .map(|line| {
            let l = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            ((l[0], l[1]), (l[2], l[3]), (l[4], l[5]))
        })
        .filter(|&(a, b, c)| contains_origin(a, b, c))
        .count()
);

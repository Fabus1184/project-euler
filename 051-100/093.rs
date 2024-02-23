#!/usr/bin/env rust-script

// cargo-deps: itertools = "0.10.5", fraction = "0.13.1"

use fraction::Fraction as D;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

let mut map: HashMap<[D; 4], HashSet<D>> = HashMap::new();
for (a, b, c, d) in itertools::iproduct!(1..10, 1..10, 1..10, 1..10) {
    for (x, y, z) in itertools::iproduct!(
            [Op::Add, Op::Sub, Op::Mul, Op::Div],
            [Op::Add, Op::Sub, Op::Mul, Op::Div],
            [Op::Add, Op::Sub, Op::Mul, Op::Div]
    ) {        
        let eval = |i, o, j| match o {
            Op::Add => i + j,
            Op::Sub => i - j,
            Op::Mul => i * j,
            Op::Div => i / j,
        };

        let e = [a, b, c, d].into_iter().map(|x| D::from(x)).sorted().collect::<Vec<_>>().try_into().unwrap();

        map.entry(e).or_default().insert(eval(eval(eval(D::from(a), x, D::from(b)), y, D::from(c)), z, D::from(d)));
        map.entry(e).or_default().insert(eval(eval(D::from(a), x, D::from(b)), y, eval(D::from(c), z, D::from(d))));
    }
}

fn longest_interval(set: &HashSet<D>) -> i32 {
    (1..).find(|&i| !set.contains(&D::from(i))).unwrap() - 1
}

println!("{}", map.iter().max_by_key(|&(_, set)| longest_interval(set)).unwrap().0.iter().map(|x| x.to_string()).join(""));

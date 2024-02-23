#!/usr/bin/env rust-script

use std::collections::HashMap;

fn routes(x: usize, y: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if x == 0 || y == 0 {
        return 1;
    }

    if let Some(&n) = memo.get(&(x, y)) {
        return n;
    }

    let n = routes(x - 1, y, memo) + routes(x, y - 1, memo);
    memo.insert((x, y), n);
    n
}

println!("{}", routes(20, 20, &mut HashMap::new()));

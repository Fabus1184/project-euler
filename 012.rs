use std::collections::HashSet;

fn divisors(n: usize) -> HashSet<usize> {
    let mut divs = HashSet::new();
    for i in 1..(n as f64).sqrt() as usize + 1 {
        if n % i == 0 {
            divs.insert(i);
            divs.insert(n / i);
        }
    }
    divs
}

struct TriangleNumber {
    n: usize,
    value: usize,
}

impl TriangleNumber {
    fn new() -> TriangleNumber {
        TriangleNumber { n: 0, value: 0 }
    }
}

impl Iterator for TriangleNumber {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.n += 1;
        self.value += self.n;
        Some(self.value)
    }
}

println!(
    "{:?}",
    TriangleNumber::new()
        .find(|&n| divisors(n).len() > 500)
        .unwrap()
);

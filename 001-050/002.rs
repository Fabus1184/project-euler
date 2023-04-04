struct Fibonacci {
    last: usize,
    curr: usize,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { last: 0, curr: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        self.curr += self.last;
        self.last = curr;
        Some(curr)
    }
}

println!(
    "{:?}",
    Fibonacci::new()
        .take_while(|x| *x <= 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum::<usize>()
);

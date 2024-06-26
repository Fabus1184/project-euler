pub fn solution() -> u32 {
    std::iter::repeat(())
        .scan((0, 1), |state @ &mut (a, b), _| {
            *state = (b, a + b);
            Some(b)
        })
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

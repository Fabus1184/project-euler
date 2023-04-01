fn factorial(n: u8) -> u64 {
    static mut FACTORIALS: [u64; 21] = [0; 21];
    unsafe {
        if FACTORIALS[n as usize] == 0 {
            FACTORIALS[n as usize] = (1..=n as u64).product();
        }
        FACTORIALS[n as usize]
    }
}

println!(
    "{}",
    (3..1_000_000)
        .filter(|&n| n
            == n.to_string()
                .chars()
                .map(|c| factorial(c.to_digit(10).unwrap() as u8))
                .sum())
        .sum::<u64>()
);

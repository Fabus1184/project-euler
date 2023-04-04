fn nth_digit(n: usize) -> u8 {
    static mut VEC: Vec<u8> = Vec::new();
    static mut LAST: usize = 0;
    unsafe {
        while VEC.len() <= n {
            LAST += 1;
            VEC.extend(
                LAST.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8),
            );
        }
        VEC[n]
    }
}

println!(
    "{:?}",
    (0..7)
        .map(|n| nth_digit(10usize.pow(n) - 1))
        .product::<u8>()
);

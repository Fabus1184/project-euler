fn is_palindrome(n: u32) -> bool {
    n.to_string() == n.to_string().chars().rev().collect::<String>()
}

pub fn solution() -> u32 {
    itertools::iproduct!(100..1000, 100..1000)
        .map(|(a, b)| a * b)
        .filter(|&x| is_palindrome(x))
        .max()
        .unwrap()
}

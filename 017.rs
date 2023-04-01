fn spell(n: usize) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        n if n < 100 => spell(n / 10 * 10) + "-" + &spell(n % 10),
        n if n < 1000 => {
            spell(n / 100)
                + " hundred"
                + &if n % 100 == 0 {
                    "".to_string()
                } else {
                    " and ".to_string() + &spell(n % 100)
                }
        }
        n if n < 1000000 => {
            spell(n / 1000)
                + " thousand"
                + &if n % 1000 == 0 {
                    "".to_string()
                } else {
                    " ".to_string() + &spell(n % 1000)
                }
        }
        _ => panic!("n too large"),
    }
}

println!(
    "{}",
    (1..1001)
        .map(|n| spell(n).chars().filter(|c| c.is_alphabetic()).count())
        .sum::<usize>()
);

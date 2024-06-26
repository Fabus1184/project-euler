fn spell_length(n: usize) -> usize {
    let ones = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let hundreds = "hundred";
    let thousands = "thousand";

    match n {
        0 => 0,
        1..=9 => ones[n - 1].len(),
        11..=19 => teens[n - 11].len(),
        10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => tens[n / 10 - 1].len(),
        100 => ones[0].len() + hundreds.len(),
        1000 => ones[0].len() + thousands.len(),
        21..=99 => tens[n / 10 - 1].len() + ones[n % 10 - 1].len(),
        101..=999 => {
            let hundreds = ones[n / 100 - 1].len() + hundreds.len();
            let rest = spell_length(n % 100);
            if n % 100 == 0 {
                hundreds
            } else {
                hundreds + "and".len() + rest
            }
        }
        _ => unimplemented!("number too large: {n}"),
    }
}

pub fn solution() -> usize {
    (1..=1000).map(spell_length).sum()
}

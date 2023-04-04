// cargo-deps: itertools="0.10.1"

use itertools::Itertools;

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.sort();
    chars == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

println!("{:?}", 
    itertools::iproduct!(1..10000, 1..10000)
        .map(|(a, b)| (a, b, a*b, a.to_string() + &b.to_string() + &(a * b).to_string()))
        .filter(|(_, _, _, s)| is_pandigital(s))
        .map(|(_, _, x, _)| x)
        .unique()
        .sum::<u32>()
)
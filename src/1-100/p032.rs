use itertools::Itertools;

pub fn solution() -> u32 {
    (1..=9)
        .permutations(9)
        .flat_map(|cs| {
            let chars = cs.into_iter().map(|c| c.to_string()).collect::<String>();
            (1..=7)
                .flat_map(|i| (i + 1..=8).map(move |j| (i, j)))
                .filter_map(move |(i, j)| {
                    let a = chars[..i].parse::<u32>().unwrap();
                    let b = chars[i..j].parse::<u32>().unwrap();
                    let c = chars[j..].parse::<u32>().unwrap();
                    if a * b == c {
                        Some(c)
                    } else {
                        None
                    }
                })
        })
        .unique()
        .sum()
}

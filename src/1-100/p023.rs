use crate::lib::prelude::Arithmetic;

fn abundants() -> impl Iterator<Item = u64> + Clone {
    (1_u64..).filter(|&n| n.proper_divisors().sum::<u64>() > n)
}

pub fn solution() -> u64 {
    let limit = 28123;
    let abundants = abundants().take_while(|&n| n < limit).collect::<Vec<_>>();
    let mut sum = vec![false; limit as usize];

    for i in 0..abundants.len() {
        for j in i..abundants.len() {
            let sum_abundants = abundants[i] + abundants[j];
            if sum_abundants < limit {
                sum[sum_abundants as usize] = true;
            }
        }
    }

    sum.iter()
        .enumerate()
        .filter(|&(_, &b)| !b)
        .map(|(i, _)| i as u64)
        .sum()
}

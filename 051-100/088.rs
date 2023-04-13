use std::collections::HashSet;

fn factor(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    factor_helper(n, k, 2, &mut vec![], &mut result);
    result
}

fn factor_helper(n: usize, k: usize, start: usize, factors: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if n == 1 && factors.len() <= k && factors.len() > 1 {
        result.push(factors.clone());
        return;
    }
    for i in start..=n {
        if n % i == 0 {
            factors.push(i);
            factor_helper(n / i, k, i, factors, result);
            factors.pop();
        }
    }
}

let mut sum = HashSet::new();
for k in 2..=12_000 {
    for i in k.. {
        if let Some(f) = factor(i, k).iter().find(|&f| {
            f.iter().product::<usize>() - f.iter().sum::<usize>() == k - f.len()
        }) {
            sum.insert(f.iter().product::<usize>());
            break;
        }
    }
}
println!("{}", sum.iter().sum::<usize>());

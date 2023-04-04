fn continued_fraction_sqrt(x: u64) -> Option<Vec<u64>> {
    let mut a = Vec::new();
    let mut m = 0;
    let mut d = 1;
    let a0 = (x as f64).sqrt() as u64;

    a.push(a0);

    let mut seen = std::collections::HashMap::new();
    seen.insert((m, d, a0), 0);

    loop {
        m = d * a.last().unwrap() - m;
        d = (x - m * m) / d;
        if d == 0 {
            return None;
        }
        let next_a = (a0 + m) / d;
        a.push(next_a);
        if let Some(&i) = seen.get(&(m, d, next_a)) {
            return Some(a[i + 1..].to_vec());
        }
        seen.insert((m, d, next_a), a.len() - 1);
    }
}

println!(
    "{}",
    (1..=10_000)
        .filter_map(|x| continued_fraction_sqrt(x))
        .filter(|x| x.len() % 2 == 1)
        .count()
);

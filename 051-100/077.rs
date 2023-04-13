// cargo-deps: primal = "0.3.2"

fn coin_sums(n: usize, atoms: &Vec<usize>) -> usize {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for &atom in atoms {
        for i in atom..=n {
            dp[i] += dp[i - atom];
        }
    }
    dp[n]
}

println!("{}", (1..)
    .skip_while(|&x| coin_sums(x, &Vec::from_iter(primal::Primes::all().take_while(|&p| p <= x))) <= 5_000)
    .next()
    .unwrap()
);

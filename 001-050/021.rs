fn amicable(n: usize) -> bool {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    let mut sum2 = 0;
    for i in 1..sum {
        if sum % i == 0 {
            sum2 += i;
        }
    }
    sum2 == n && sum != n
}

println!("{}", (1..10000).filter(|&x| amicable(x)).sum::<usize>());

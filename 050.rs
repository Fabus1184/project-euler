// cargo-deps: primal="0.3.2"

let mut max = (1, 1);

let primes = primal::Primes::all()
    .take_while(|&x| x < 1_000_000)
    .collect::<Vec<_>>();

for i in 1..primes.len() {
    let ws = primes
        .windows(i)
        .map(|w| (w, w.iter().sum::<usize>()))
        .collect::<Vec<_>>();
    if let Some((_, s)) = ws.first() {
        if s > &1_000_000 {
            break;
        }
        for &(w, s) in ws
            .iter()
            .filter(|(_, s)| s < &&1_000_000)
            .filter(|(_, s)| primal::is_prime((*s).try_into().unwrap()))
        {
            if w.len() > max.0 {
                max = (w.len(), s);
            }
        }
    }
}

println!("{}", max.1);

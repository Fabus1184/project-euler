// cargo-deps: itertools = "0.10.5"

use itertools::Itertools;

let rs = [
    (0..).map(|n| n * n).skip_while(|&n| n < 1000).take_while(|&n| n < 10000).collect::<Vec<_>>(),
    (0..).map(|n| n * (n + 1) / 2).skip_while(|&n| n < 1000).take_while(|&n| n < 10000).collect::<Vec<_>>(),
    (0..).map(|n| n * (3 * n - 1) / 2).skip_while(|&n| n < 1000).take_while(|&n| n < 10000).collect::<Vec<_>>(),
    (0..).map(|n| n * (2 * n - 1)).skip_while(|&n| n < 1000).take_while(|&n| n < 10000).collect::<Vec<_>>(),
    (0..).map(|n| n * (5 * n - 3) / 2).skip_while(|&n| n < 1000).take_while(|&n| n < 10000).collect::<Vec<_>>(),
    (0..).map(|n| n * (3 * n - 2)).skip_while(|&n| n < 1000).take_while(|&n| n < 10000).collect::<Vec<_>>()
];

fn first(n: u64) -> u64 {
    n / 100
}

fn last(n: u64) -> u64 {
    n % 100
}

for v in (0..6).permutations(6) {
    let [a, b, c, d, e, f] = v[..] else { panic!() };
    
    if [a, b, c, d, e, f].iter().dedup().count() != 6 {
        continue;
    }

    for &r1 in rs[a].iter() {
        for &r2 in rs[b].iter().filter(|&&r2| last(r1) == first(r2) && ![r1].contains(&r2)) {
            for &r3 in rs[c].iter().filter(|&&r3| last(r2) == first(r3) && ![r1, r2].contains(&r3)) {
                for &r4 in rs[d].iter().filter(|&&r4| last(r3) == first(r4) && ![r1, r2, r3].contains(&r4)) {
                    for &r5 in rs[e].iter().filter(|&&r5| last(r4) == first(r5) && ![r1, r2, r3, r4].contains(&r5)) {
                        for &r6 in rs[f].iter().filter(|&&r6| last(r5) == first(r6) && last(r6) == first(r1) && ![r1, r2, r3, r4, r5].contains(&r6)) {
                            println!("{}", [r1, r2, r3, r4, r5, r6].iter().sum::<u64>());
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

}
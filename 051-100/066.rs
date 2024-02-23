#!/usr/bin/env rust-script

fn convergents(a_n: impl Iterator<Item = u128>) -> impl Iterator<Item = (u128, u128)> {
    let (mut a, mut b) = (1, 0);
    let (mut c, mut d) = (0, 1);
    a_n.map(move |n| {
        let (a_k, b_k) = (a, b);
        let (c_k, d_k) = (c, d);
        a = c_k;
        b = d_k;
        c = a_k + n * c_k;
        d = b_k + n * d_k;
        (d, c)
    })
}

fn sqrt_continued_fraction(x: u128) -> impl Iterator<Item = u128> {
    let mut m = 0;
    let mut d = 1;
    let mut a = (x as f64).sqrt().floor() as u128;
    let a0 = a;
    std::iter::once(a0).chain(std::iter::from_fn(move || {
        m = d * a - m;
        d = (x - m * m) / d;
        a = (a0 + m) / d;
        Some(a)
    }))
}

fn pell_fundamental(n: u128) -> (u128, u128) {
    convergents(sqrt_continued_fraction(n))
        .filter(|(h, k)| h * h - n * k * k == 1)
        .next()
        .unwrap()
}

println!(
    "{:?}",
    (2..=1000)
        .filter(|&i| (i as f64).sqrt().floor().powf(2.0) != i as f64)
        .max_by_key(|&d| pell_fundamental(d).0)
        .unwrap()
);

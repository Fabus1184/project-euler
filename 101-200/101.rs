#!/usr/bin/env rust-script

fn polyfit(values: Vec<(i64, i64)>) -> impl Fn(i64) -> i64 {
    move |x| {
        (0..values.len())
            .map(|i| {
                values[i].1 as f64
                    * (0..values.len())
                        .filter(|&j| j != i)
                        .map(|j| (x - values[j].0) as f64 / (values[i].0 - values[j].0) as f64)
                        .product::<f64>()
            })
            .sum::<f64>()
            .round() as i64
    }
}

fn f(n: i64) -> i64 {
    1 - n + n.pow(2) - n.pow(3) + n.pow(4) - n.pow(5) + n.pow(6) - n.pow(7) + n.pow(8) - n.pow(9) + n.pow(10)
}

fn first_incorrect_term(f: impl Fn(i64) -> i64, poly: impl Fn(i64) -> i64) -> i64 {
    (1..).find(|&n| f(n) != poly(n)).map(poly).unwrap()
}

let mut sum = 0;
for degree in 0..10 {
    let poly = polyfit((1..).map(|x| (x, f(x))).take(degree + 1).collect::<Vec<_>>());
    sum += first_incorrect_term(f, poly);
}
println!("{}", sum);

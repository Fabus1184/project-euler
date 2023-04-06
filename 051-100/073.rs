// cargo-deps: fraction = "0.13.1"

use fraction::ToPrimitive;

type F = fraction::GenericFraction<u64>;

fn gcd(a: F, b: F) -> u64 {
    if *b.numer().unwrap() == 0 {
        a.numer().unwrap().to_u64().unwrap()
    } else {
        gcd(b, a % b)
    }
}

let mut n: u64 = 0;

for i in 1u64..=12_000 {
    let mut n1 = F::new(((1f64 / 3f64) * i as f64).floor() as u64, i);
    let n2 = F::new(((1f64 / 2f64) * i as f64).ceil() as u64, i);
    let ix = F::new(1u64, i);
    
    if n1 + ix >= n2 {
        continue;
    }

    n1 += ix;

    while n1 < n2 {
        if gcd(F::from(i), n1 * F::from(i)) == 1 {
            n += 1;
        }
        n1 += ix;
    }
}

println!("{}", n);

let mut n = 0;

for i in 1.. {
    for j in 1..=2*i {
        if ((i * i + j * j) as f64).sqrt() % 1.0 == 0.0 {
            if j > i + 1 {
                n += (i + i + 2 - j) / 2;
            } else {
                n += j / 2;
            }
        }
    }

    if n > 1_000_000 {
        println!("{}", i);
        break;
    }
}
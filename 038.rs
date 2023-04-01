fn is_pandigital(n: u64) -> bool {
    if n.to_string().len() != 9 {
        return false;
    }

    let mut digits = [false; 10];
    let mut n = n;
    while n > 0 {
        let digit = (n % 10) as usize;
        if digit == 0 || digits[digit] {
            return false;
        }
        digits[digit] = true;
        n /= 10;
    }
    digits[1..].iter().all(|&x| x)
}


let mut max = 0;
for i in 1..10000 {
    let mut n = "".to_string();
    let mut j = 1;
    while n.len() < 9 {
        n += &(i * j).to_string();
        j += 1;
    }
    if n.len() == 9 && is_pandigital(n.parse().unwrap()) {
        max = max.max(n.parse().unwrap());
    }
}
println!("{}", max);

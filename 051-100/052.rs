use std::collections::HashSet;

for i in 1.. {
    let s = i.to_string().chars().collect();
    for n in [2, 3, 4, 5, 6] {
        if (i * n).to_string().chars().collect::<HashSet<_>>() != s {
            break;
        }

        if n == 6 {
            println!("{}", i);
            return Ok(());
        }
    }
}
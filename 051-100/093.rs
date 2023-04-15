// cargo-deps: eval = "0.4.3", itertools = "0.10.5"

use eval::Value::Number;
use itertools::Itertools;
use std::collections::HashSet;

let mut max = (0, 0);

for a in 1..7 {
    for b in a+1..8 {
        for c in b+1..9 {
            for d in c+1..10 {
                let mut set = HashSet::new();
                
                for os in ["+", "-", "*", "/"].into_iter().combinations(3) {
                    for ps in itertools::iproduct!(0..2, 0..2, 0..2, 0..2, 0..2, 0..2) {
                        let e = format!("
                        {}
                            {}
                                {} {} {}
                            {}
                                {}
                                {}
                        {}
                            ",
                            if ps.1 == 0 { "(" } else { "" },
                            if ps.0 == 0 { "(" } else { "" },
                            a, os[0], b,
                            if ps.0 == 0 { ")" } else { "" },
                            os[1], c,
                            if ps.1 == 0 { ")" } else { "" },
                        );
                        
                        if let Ok(Number(y)) = eval::eval(e.as_str()) {
                            if let Some(x) = y.as_i64() {
                                println!("{} = {}", e, x);
                                set.insert(x);
                            }
                        }
                    }
                }

                if set.len() > max.0 {
                    max = (set.len(), a * 1000 + b * 100 + c * 10 + d);
                }
            }
        }
    }
}

println!("{:?}", max);
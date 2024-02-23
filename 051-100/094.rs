#!/usr/bin/env rust-script

// https://oeis.org/A103772
// https://oeis.org/A103974

println!(
    "{:?}",
    [5, 65, 901, 12_545, 174_725, 2_433_601, 33_895_685]
        .into_iter()
        .map(|a| 3 * a + 1)
        .sum::<u32>()
        + [17, 241, 3_361, 46_817, 652_081, 9_082_321, 126_500_417]
            .into_iter()
            .map(|a| 3 * a - 1)
            .sum::<u32>()
);

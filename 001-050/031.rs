#!/usr/bin/env rust-script

const COINS: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn number_of_ways(n: u32) -> u32 {
    let mut ways = vec![0; n as usize + 1];
    ways[0] = 1;
    for &coin in COINS.iter() {
        for i in coin as usize..n as usize + 1 {
            ways[i] += ways[i - coin as usize];
        }
    }
    ways[n as usize]
}

println!("{:?}", number_of_ways(200));

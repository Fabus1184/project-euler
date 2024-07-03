fn coinsums(target: u32, coins: &[u32]) -> u32 {
    let mut ways = vec![0; target as usize + 1];
    ways[0] = 1;
    for &coin in coins {
        for i in coin as usize..=target as usize {
            ways[i] += ways[i - coin as usize];
        }
    }
    ways[target as usize]
}

pub fn solution() -> u32 {
    coinsums(200, &[1, 2, 5, 10, 20, 50, 100, 200])
}

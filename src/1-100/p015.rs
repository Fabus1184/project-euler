fn lattice_paths(n: u64) -> u64 {
    let mut paths = vec![1; n as usize + 1];
    for _ in 0..n {
        for j in 1..n as usize + 1 {
            paths[j] += paths[j - 1];
        }
    }
    paths[n as usize]
}

pub fn solution() -> u64 {
    lattice_paths(20)
}

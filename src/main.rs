use std::collections::HashMap;

use anyhow::Context;

include!(concat!(env!("OUT_DIR"), "/mods.rs"));

mod lib;

fn main() -> anyhow::Result<()> {
    let phpsessid = std::env::var("PHPSESSID").context("PHPSESSID is not set")?;
    let keep_alive = std::env::var("KEEP_ALIVE").context("KEEP_ALIVE is not set")?;

    let client = lib::solution::client(phpsessid, keep_alive)?;

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let problem = match args.as_slice() {
        [p] => Some(p.parse::<u32>()?),
        _ => None,
    };

    let mut cache = serde_json::from_str::<HashMap<u32, String>>(
        &std::fs::read_to_string("cache.json").unwrap_or_default(),
    )
    .unwrap_or_default();

    for (number, solution) in SOLUTIONS
        .into_iter()
        .filter(|&(n, _)| problem.map_or(true, |p| p == n))
    {
        let time = std::time::Instant::now();
        let result = solution();
        let elapsed = time.elapsed();
        println!("Problem {}: {} ({:.2?})", number, result, elapsed);

        let text = cache
            .get(&number)
            .cloned()
            .ok_or(anyhow::anyhow!("none"))
            .or_else(|_| {
                let solution = lib::solution::solution(&client, number)?;
                cache.insert(number, solution.clone());
                anyhow::Ok(solution)
            })?;

        anyhow::ensure!(
            format!("{result}") == text,
            "Solution does not match: {} != {}",
            result,
            text
        );
    }

    std::fs::write("cache.json", serde_json::to_string(&cache)?)?;

    Ok(())
}

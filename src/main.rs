include!(concat!(env!("OUT_DIR"), "/mods.rs"));

mod lib;

fn main() -> anyhow::Result<()> {
    let client = lib::solution::client(
        
    )?;

    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let problem = match args.as_slice() {
        [p] => Some(p.parse::<u32>()?),
        _ => None,
    };

    for (number, solution) in SOLUTIONS
        .into_iter()
        .filter(|&(n, _)| problem.map_or(true, |p| p == n))
    {
        let time = std::time::Instant::now();
        let result = solution();
        let elapsed = time.elapsed();
        println!("Problem {}: {} ({:.2?})", number, result, elapsed);

        let text = lib::solution::solution(&client, number)?;
        anyhow::ensure!(
            format!("{result}") == text,
            "Solution does not match: {} != {}",
            result,
            text
        );
    }

    Ok(())
}

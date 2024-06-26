include!(concat!(env!("OUT_DIR"), "/mods.rs"));

mod lib;

fn main() {
    for (name, solution) in SOLUTIONS {
        let time = std::time::Instant::now();
        let result = solution();
        let elapsed = time.elapsed();
        println!("{}: {} ({:.2?})", name, result, elapsed);
    }
}

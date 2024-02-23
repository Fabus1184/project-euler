use std::{os::unix::fs::PermissionsExt, process::Command, str::FromStr, sync::Arc};

use anyhow::Context;
use reqwest::{
    blocking::{Client, ClientBuilder},
    cookie::Jar,
    Url,
};
use walkdir::WalkDir;

const EXTENSIONS: &[&str] = &["rs", "hs"];

const BASE_URL: &str = "https://www.projecteuler.net";

fn fetch_solution(client: &Client, problem: u32) -> anyhow::Result<i128> {
    let url = format!("{}/problem={}", BASE_URL, problem);
    let resp = client.get(url).send().unwrap().text().unwrap();

    let t = resp
        .split("<div>Answer:&nbsp;&nbsp;<span class=\"strong\">")
        .nth(1)
        .ok_or(anyhow::anyhow!("Failed to parse response: {}", resp))?;

    let t = t
        .split("</span>")
        .next()
        .ok_or(anyhow::anyhow!("Failed to parse response: {}", resp))?;

    t.parse::<i128>().context("Failed to parse response")
}

fn main() -> anyhow::Result<()> {
    let dir = std::env::args().nth(1).unwrap_or(".".to_string());

    let psid = "784d8ba843c063e157783d286ddc4143";
    let keep_alive = "1707450248%232009713%237Zk7s0GtCzXTqakHlo0akTiqDzydzXid";

    let url = Url::from_str(BASE_URL)?;

    /*let psid = std::io::stdin()
    .lines()
    .next()
    .context("Failed to read input")?
    .context("No input")?;*/

    let cookie_store = Arc::new(Jar::default());
    cookie_store.add_cookie_str(
        format!("PHPSESSID={psid}; Domain=projecteuler.net").as_str(),
        &url,
    );
    cookie_store.add_cookie_str(
        format!("keep_alive={keep_alive}; Domain=projecteuler.net").as_str(),
        &url,
    );

    let client = ClientBuilder::new()
        .cookie_provider(cookie_store)
        .build()
        .context("Failed to build client")?;

    let mut files = WalkDir::new(dir)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let metadata = e.metadata().ok()?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 != 0 {
                Some(e)
            } else {
                None
            }
        })
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| EXTENSIONS.contains(&e))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .map(|f| {
            let number = f
                .file_name()
                .context(format!("Failed to get file name: {}", f.display()))?
                .to_str()
                .context(format!(
                    "Failed to convert file name to string: {}",
                    f.display()
                ))?
                .split('.')
                .next()
                .context(format!("Failed to split file name: {}", f.display()))?
                .parse::<u32>()
                .context(format!("Failed to parse file name: {}", f.display()))?;

            Ok((number, f))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    files.sort_by_key(|(number, _)| *number);

    for (number, file) in files {
        let mut process = Command::new(&file);

        let start = std::time::Instant::now();
        let ret = process.output().context("Failed to execute process")?;
        let elapsed = start.elapsed();

        if !ret.status.success() {
            println!("Failed to execute {}", file.display());
            break;
        }

        let stdout = std::str::from_utf8(&ret.stdout)
            .context(format!("Failed to parse output: {:?}", ret.stdout))?
            .trim_end_matches('\n');

        let result = stdout
            .parse::<i128>()
            .context(format!("Failed to parse output: {}", stdout))?;

        let solution = fetch_solution(&client, number)
            .context(format!("Failed to fetch solution for problem {}", number))?;

        if result != solution {
            println!(
                "Failed to solve problem {}, expected {:?}, got {}",
                number, solution, stdout
            );
            break;
        }

        println!(
            "Problem {} solved: {} ({}ms)",
            number,
            result,
            elapsed.as_millis()
        );
    }

    Ok(())
}

use anyhow::Context;

pub fn client(phpsessid: String, keep_alive: String) -> anyhow::Result<reqwest::blocking::Client> {
    reqwest::blocking::ClientBuilder::new()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&format!(
                    "PHPSESSID={}; keep_alive={}",
                    phpsessid, keep_alive
                ))?,
            );
            headers
        })
        .build()
        .context("Failed to build client")
}

pub fn solution(client: &reqwest::blocking::Client, n: u32) -> anyhow::Result<String> {
    let text = client
        .get(format!("https://projecteuler.net/problem={}", n))
        .send()
        .context("Failed to send request")?
        .text()
        .context("Failed to get response")?;

    text.split(r#"<div>Answer:&nbsp;&nbsp;<span class="strong">"#)
        .nth(1)
        .context("Failed to find answer")
        .and_then(|x| x.split("</span>").next().context("Failed to find answer"))
        .map(|x| x.to_string())
        .context("Failed to parse answer")
}

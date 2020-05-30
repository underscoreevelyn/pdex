use std::collections::HashMap;
use serde::Deserialize;
use reqwest::blocking::Client;
use std::time::Duration;

#[derive(Deserialize)]
struct ParseResponse {
    wikitext: HashMap<String, String>,
}

pub fn get_page_contents(target: &str) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION"),
            ))
        .build().ok()?;

    let resp = client.get("https://bulbapedia.bulbagarden.net/w/api.php")
        .query(&[
               ("format", "json"),
               ("redirects", "on"), 
               ("action", "parse"), 
               ("prop", "wikitext"), 
               ("page", &target)
        ]).send().ok()?;

    let json = resp.json::<HashMap<String, ParseResponse>>().ok()?;
    let wikitext = json.get("parse")?.wikitext.get("*")?;

    Some(wikitext.to_string())
}

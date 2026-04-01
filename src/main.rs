use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct AsyncCommandResponse {
    pub title: Option<String>,
    pub content: Option<String>,
    pub next_content: Option<String>,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        print_error(&e.to_string());
    }
}
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_error("Please enter a search term.");
        return Ok(());
    }

    let bindings = args.join(" ");
    let keyword = urlencoding::encode(&bindings);

    let search_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&format=json&list=search&srsearch={}&srlimit=1",
        keyword
    );

    let mut res = surf::get(&search_url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Cache-Control", "no-cache")
        .await
        .map_err(|e| format!("Wikipedia Fetch Error: {}", e))?;

    let body_str = res
        .body_string()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let body: serde_json::Value = serde_json::from_str(&body_str)
        .map_err(|e| format!("JSON Parse Error: {}\nRaw Body: {}", e, body_str))?;

    let search_results = body["query"]["search"]
        .as_array()
        .filter(|list| !list.is_empty())
        .ok_or("No Wikipedia results found for this query.")?;

    let pageid = search_results[0]["pageid"]
        .as_u64()
        .ok_or("Wikipedia returned an invalid Page ID.")?;

    let document_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&pageids={}&exintro&explaintext",
        pageid
    );

    let mut res = surf::get(&document_url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/json")
        .header("Accept-Language", "en-US,en;q=0.9")
        .await
        .map_err(|e| format!("Content fetch failure: {}", e))?;

    let body: serde_json::Value = res
        .body_json()
        .await
        .map_err(|_| "Failed to parse page content.")?;

    let pages = body["query"]["pages"]
        .as_object()
        .ok_or("Malformed Wikipedia response (missing pages).")?;

    let page = pages
        .get(&pageid.to_string())
        .ok_or("Page content not found in Wikipedia response.")?;

    let title = page["title"].as_str().map(|s| s.to_string());
    let content = page["extract"].as_str().map(|s| s.to_string());

    if title.is_none() && content.is_none() {
        return Err("Wikipedia page exists but contains no readable content.".into());
    }

    let response = AsyncCommandResponse {
        title,
        content,
        next_content: None,
    };

    println!("{}", serde_json::to_string(&response)?);

    Ok(())
}

fn print_error(msg: &str) {
    let err = json!({
        "title": "Wikipedia Search",
        "content": format!("⚠️ {}", msg)
    });
    // Ensure we only print one thing to stdout so the parser doesn't get confused
    println!("{}", err.to_string());
}

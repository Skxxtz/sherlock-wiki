use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use surf;

#[derive(Debug, Serialize, Deserialize)]
struct JsonResponse {
    title: String,
    content: String,
    next_content: String,
}
impl JsonResponse {
    fn show(&self) {
        let s = serde_json::to_string(self).unwrap();
        println!("{}", s);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let keyword = &args[1];
    let search_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&format=json&list=search&srsearch={}&srlimit=1",
        keyword
    );

    // Make the GET request
    let mut response = surf::get(&search_url).await?;

    // Parse the response body as JSON
    let body: Value = response.body_json().await?;

    // Extract the `pageid` of the first result
    let pageid = body["query"]["search"]
        .as_array()
        .and_then(|search_results| search_results.get(0))
        .and_then(|first_result| first_result.get("pageid"))
        .and_then(Value::as_u64)
        .unwrap();

    let document_url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts|pageimages&pageids={}&exintro&explaintext",
        pageid
    );

    let mut response = surf::get(&document_url).await?;

    // Parse the response body as JSON
    let body: Value = response.body_json().await?;

    // Extract the `pageid` of the first result
    //
    //
    if let Some(page) = body["query"]["pages"].get(pageid.to_string()) {
        let title = page["title"].as_str().unwrap_or("").to_owned();
        let content = page["extract"].as_str().unwrap_or("").to_owned();
        let next_content = page["extract"].as_str().unwrap_or("").to_owned();
        let r = JsonResponse {
            title,
            content,
            next_content,
        };
        r.show();
    };
    Ok(())
}

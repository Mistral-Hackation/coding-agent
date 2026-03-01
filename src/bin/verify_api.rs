// use reqwest::header::USER_AGENT;
// use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = std::env::var("SERPER_API_KEY")?;

    println!("Checking SerpApi access...");
    let url = format!("https://serpapi.com/search.json?q=test&api_key={}", key);

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;

    println!("Status: {}", resp.status());
    let text = resp.text().await?;
    println!("Response: {:.200}", text); // print first 200 chars

    Ok(())
}

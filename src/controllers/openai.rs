use reqwest::Client;
use crate::models::ChatRequest;

pub async fn gpt_response(chat_request: ChatRequest, url: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let payload = serde_json::to_value(chat_request)?;

    let client: Client = Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;
    
    let response_text = response.text().await?;
    println!("{}", response_text);
        
    Ok(response_text)
}

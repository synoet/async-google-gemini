use anyhow::Result;
use async_google_gemini::{client::Client, config::GeminiConfig, types::claude};

#[tokio::main]
async fn main() -> Result<()> {
    let config = GeminiConfig::try_from_service_account_env()?;
    let client = Client::new(config)?;

    let req = claude::RawPredictRequest::builder()
        .max_tokens(300)
        .stream(false)
        .messages(vec![claude::ClaudeMessage {
            role: "user".to_string(),
            content: "Write me a poem about crabs".to_string(),
        }])
        .build()?;

    let response = client
        .claude()
        .raw_predict(claude::ClaudeModel::Claude35SonnetV2, req)
        .await?;

    dbg!(response);
    Ok(())
}

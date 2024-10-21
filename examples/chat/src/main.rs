use anyhow::Result;
use async_google_gemini::{
    client::Client,
    config::GeminiConfig,
    types::{
        content::{Content, GenerateContentRequest, Part, TextPart},
        gemini::GeminiModel,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = GeminiConfig::try_from_service_account_env()?;
    let client = Client::new(config)?;

    let req = GenerateContentRequest::builder()
        .contents(vec![Content {
            role: "user".to_string(),
            parts: vec![Part::TextPart(TextPart {
                text: "Write me a poem about crabs".to_string(),
            })],
        }])
        .build()?;

    let response = client
        .gemini()
        .generate_content(GeminiModel::Gemini15Flash002, req)
        .await?;

    dbg!(response);
    Ok(())
}

use anyhow::Result;
use async_google_gemini::{
    client::Client,
    config::GeminiConfig,
    types::{
        content::{Content, GenerateContentRequest, Part, TextPart},
        gemini::GeminiModel,
    },
};
use futures::StreamExt as _;

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

    let mut stream = client
        .gemini()
        .stream_generate_content(GeminiModel::Gemini15Flash002, req)
        .await?;

    while let Some(message) = stream.next().await {
        let response = message?;
        println!("{:?}", response);
    }

    Ok(())
}

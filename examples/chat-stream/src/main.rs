use anyhow::Result;
use async_google_gemini::{
    client::GeminiClient,
    types::{
        chat_request::{
            Content, GeminiChatRequest, GenerationConfig, Part, PartData, SafetySetting,
        },
        model::GeminiModel,
    },
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client = GeminiClient::new()?;

    let request = GeminiChatRequest::builder()
        .contents(vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                data: PartData::Text("Write me a poem about crabs".to_string()),
                video_metadata: None,
            }],
        }])
        .model(GeminiModel::Gemini15Pro002)
        .safety_settings(SafetySetting::default())
        .generation_config(GenerationConfig::default())
        .build()?;

    let mut stream = client.chat().create_stream(request).await;

    while let Some(message) = stream.next().await {
        println!("{:?}", message);
    }

    Ok(())
}

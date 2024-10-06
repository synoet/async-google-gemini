# async-google-gemini

async rust library for interacting with google's gemini API

*heavily inspired by [async-openai](https://github.com/64bit/async-openai)*

### Features

- [x] Generating Content With Streaming [(example)](examples/chat-stream)
- [x] Generating Content Without Streaming [(example)](examples/chat)
- [ ] Multi-Modal Generation With Streaming
- [ ] Multi-Modal Generation Without Streaming
- [ ] Support Service Accounts
- [ ] Function calling
- [ ] Count tokens

### Usage

```bash
export GEMINI_PROJECT_ID=your-project-id
export GEMINI_LOCATION=your-location
export GEMINI_API_KEY=your-api-key # can be obtained with gcloud auth print access_token
```

```rust
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
                data: PartData::Text("Hello, how are you?".to_string()),
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

```

More examples can be found in the [examples](examples) directory.


use std::pin::Pin;

use crate::{
    client::GeminiClient,
    error::GeminiError,
    types::{
        chat_request::GeminiChatRequest, chat_response::GeminiChatResponse, model::GeminiModel,
    },
};
use futures::{stream::StreamExt, Stream};
use tokio::sync::mpsc;
pub struct Chat<'c> {
    client: &'c GeminiClient,
}

#[derive(PartialEq)]
enum ChunkType {
    Start,
    Comma,
    End,
}

impl<'c> Chat<'c> {
    pub fn new(client: &'c GeminiClient) -> Self {
        Self { client }
    }

    pub async fn create_stream(
        &self,
        request: GeminiChatRequest,
    ) -> Pin<Box<dyn Stream<Item = Result<GeminiChatResponse, GeminiError>> + Send + 'static>> {
        let model = request
            .model
            .as_ref()
            .unwrap_or(&GeminiModel::Gemini15Pro002);

        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent",
            self.client.config.location,
            self.client.config.project_id,
            self.client.config.location,
            model.to_string(),
        );

        let client = self.client.http_client.clone();

        let (wx, rx) = mpsc::unbounded_channel();
        let api_key = self.client.config.api_key.clone();

        tokio::spawn(async move {
            let res = match client
                .post(&url)
                .header("content-type", "application/json; charset=utf-8")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .send()
                .await
            {
                Ok(res) => res,

                Err(e) => {
                    if let Err(_) = wx.send(Err(GeminiError::RequestError(e))) {
                        return;
                    }
                    return;
                }
            };

            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = match String::from_utf8(chunk.unwrap().to_vec()) {
                    Ok(c) => c,
                    Err(e) => {
                        if let Err(_) = wx.send(Err(GeminiError::ParseError(e.to_string()))) {
                            break;
                        }
                        return;
                    }
                };
                let mut lines: Vec<String> =
                    chunk.lines().map(|c| c.to_string()).collect::<Vec<_>>();

                let chunk_type = match lines[0].as_str() {
                    "[{" => {
                        let mut new_line = lines.remove(0).to_string();
                        new_line = new_line[1..new_line.len()].to_string();
                        lines.insert(0, new_line);
                        ChunkType::Start
                    }
                    "," => {
                        lines.remove(0);
                        ChunkType::Comma
                    }
                    "]" => ChunkType::End,
                    _ => continue,
                };

                if chunk_type == ChunkType::End {
                    break;
                }

                let content = lines.join("");

                let res = match serde_json::from_str::<GeminiChatResponse>(&content) {
                    Ok(c) => c,
                    Err(e) => {
                        if let Err(_) = wx.send(Err(GeminiError::ParseError(e.to_string()))) {
                            break;
                        }
                        return;
                    }
                };

                if let Err(_) = wx.send(Ok(res)) {
                    break;
                }
            }
        });

        Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
    }
}

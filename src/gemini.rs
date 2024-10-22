use std::pin::Pin;

use crate::{client::Client, error::GeminiError, types::content::GenerateContentErrorResponse};
use futures::{stream::StreamExt, Stream};
use serde_json::Value;
use tokio::sync::mpsc;

use crate::types::{
    content::{GenerateContentRequest, GenerateContentResponse},
    gemini::GeminiModel,
};

pub struct Gemini<'c> {
    client: &'c Client,
}

#[derive(PartialEq)]
enum ChunkType {
    Start,
    Comma,
    End,
}

impl<'c> Gemini<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a chat response
    pub async fn generate_content(
        &self,
        model: GeminiModel,
        request: GenerateContentRequest,
    ) -> Result<GenerateContentResponse, GeminiError> {
        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
            self.client.config.location(),
            self.client.config.project_id(),
            self.client.config.location(),
            model.to_string(),
        );

        let client = self.client.http_client.clone();

        let token = self.client.config.token().await.unwrap();

        let res = match client
            .post(&url)
            .header("content-type", "application/json; charset=utf-8")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await
        {
            Ok(res) => res,

            Err(e) => {
                return Err(e.into());
            }
        };

        let json = res.json::<serde_json::Value>().await?;

        let response = serde_json::from_value::<GenerateContentResponse>(json)
            .map_err(|e| GeminiError::ParseError(format!("failed to parse response: {}", e)))?;

        Ok(response)
    }

    /// Create a chat stream response
    /// partial message deltas will be sent as stream chunks
    pub async fn stream_generate_content(
        &self,
        model: GeminiModel,
        request: GenerateContentRequest,
    ) -> Pin<Box<dyn Stream<Item = Result<GenerateContentResponse, GeminiError>> + Send + 'static>>
    {
        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent",
            self.client.config.location(),
            self.client.config.project_id(),
            self.client.config.location(),
            model.to_string(),
        );

        let client = self.client.http_client.clone();

        let token = self.client.config.token().await.unwrap();

        let (wx, rx) = mpsc::unbounded_channel();
        let api_key = token;

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
                    if let Err(_) = wx.send(Err(e.into())) {
                        return;
                    }
                    return;
                }
            };

            let mut stream = res.bytes_stream();
            let mut pending_chunk: String = String::new();
            while let Some(chunk) = stream.next().await {
                let mut chunk = match String::from_utf8(chunk.unwrap().to_vec()) {
                    Ok(c) => c,
                    Err(e) => {
                        if let Err(_) = wx.send(Err(GeminiError::ParseError(e.to_string()))) {
                            break;
                        }
                        return;
                    }
                };

                if !pending_chunk.is_empty() {
                    chunk = pending_chunk.clone() + &chunk;
                }

                let mut lines: Vec<String> =
                    chunk.lines().map(|c| c.to_string()).collect::<Vec<_>>();

                // An error message will be a single chunk with both a start and end token
                // need to extract the inner json object with error message and try and parse it
                if lines[0].as_str().starts_with("[")
                    && lines[lines.len() - 1].as_str().ends_with("]")
                {
                    let error_content = lines
                        .join("")
                        .trim_matches(|c| c == '[' || c == ']')
                        .trim()
                        .to_string();

                    match serde_json::from_str::<GenerateContentErrorResponse>(&error_content) {
                        Ok(c) => {
                            if let Err(_) = wx.send(Err(c.into())) {
                                break;
                            }
                            return;
                        }
                        Err(e) => {
                            if let Err(_) = wx.send(Err(GeminiError::ParseError(format!(
                                "failed to parse error message: {}",
                                e
                            )))) {
                                break;
                            }
                            return;
                        }
                    };
                }

                let chunk_type = match lines[0].as_str() {
                    "[{" => {
                        let mut new_line = lines.remove(0).to_string();
                        new_line = new_line[1..new_line.len()].to_string();
                        lines.insert(0, new_line);
                        pending_chunk = String::new();
                        ChunkType::Start
                    }
                    "," => {
                        lines.remove(0);
                        pending_chunk = String::new();
                        ChunkType::Comma
                    }
                    "]" => ChunkType::End,
                    _ => {
                        continue;
                    }
                };

                if chunk_type == ChunkType::End {
                    break;
                }

                let content = lines.join("");

                // If the chunk ends with a comma, we need to check if the chunk is valid json
                // If it is, we parse it as json and continue to the next chunk
                // If it is not, we wait for the next chunk and combine them together
                if chunk_type == ChunkType::Comma {
                    if let Err(_) = serde_json::from_str::<Value>(content.as_str()) {
                        pending_chunk.push_str(content.as_str());
                        continue;
                    } else {
                        pending_chunk = String::new();
                    }
                }

                let res = match serde_json::from_str::<GenerateContentResponse>(&content) {
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

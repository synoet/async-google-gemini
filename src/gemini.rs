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
    #[tracing::instrument(level = "debug", skip(self))]
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
                tracing::error!(error=?e, "failed to send request to google vertex");
                return Err(e.into());
            }
        };

        let json = res.json::<serde_json::Value>().await.map_err(|e| {
            tracing::error!(error=?e, "failed to parse response from google vertex");
            GeminiError::ParseError(e.to_string())
        })?;

        let response = serde_json::from_value::<GenerateContentResponse>(json)
            .map_err(|e| GeminiError::ParseError(format!("failed to parse response: {}", e)))?;

        Ok(response)
    }

    /// Create a chat stream response
    /// partial message deltas will be sent as stream chunks
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn stream_generate_content(
        &self,
        model: GeminiModel,
        request: GenerateContentRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<GenerateContentResponse, GeminiError>> + Send + 'static>>,
        GeminiError,
    > {
        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent",
            self.client.config.location(),
            self.client.config.project_id(),
            self.client.config.location(),
            model.to_string(),
        );

        let client = self.client.http_client.clone();

        let token = self.client.config.token().await.map_err(|e| {
            tracing::error!(error=?e, "failed to get authentication token");
            GeminiError::AuthenticationError(e.to_string())
        })?;

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
                    tracing::error!(error=?e, "failed to send request to google vertex");
                    if let Err(send_error) = wx.send(Err(e.into())) {
                        tracing::error!(error=?send_error, "failed to send error message to stream");
                        return;
                    }
                    return;
                }
            };

            tracing::debug!("response from google vertex received");

            let mut stream = res.bytes_stream();
            let mut pending_chunk: String = String::new();
            while let Some(chunk) = stream.next().await {
                let raw_chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(error=?e, "failed to read chunk from google vertex");
                        if let Err(send_error) =
                            wx.send(Err(GeminiError::ParseError(e.to_string())))
                        {
                            tracing::error!(error=?send_error, "failed to send error message to stream");
                            break;
                        }
                        return;
                    }
                };

                let mut chunk = match String::from_utf8(raw_chunk.to_vec()) {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(error=?e, "failed to parse chunk while streaming from google vertex");
                        if let Err(send_error) =
                            wx.send(Err(GeminiError::ParseError(e.to_string())))
                        {
                            tracing::error!(error=?send_error, "failed to send error message to stream");
                            break;
                        }
                        return;
                    }
                };

                tracing::debug!("received chunk {}", &chunk);

                if !pending_chunk.is_empty() {
                    tracing::debug!("appending pending chunk to new chunk");
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
                            if let Err(send_error) = wx.send(Err(c.into())) {
                                tracing::error!(
                                    error=?send_error,
                                    "failed to send error message to stream"
                                );
                                break;
                            }
                            return;
                        }
                        Err(e) => {
                            if let Err(send_error) = wx.send(Err(GeminiError::ParseError(format!(
                                "failed to parse error message: {}",
                                e
                            )))) {
                                tracing::error!(
                                    error=?send_error,
                                    "failed to send error message to stream"
                                );
                                break;
                            }
                            return;
                        }
                    };
                }

                let chunk_type = match lines[0].as_str() {
                    "[{" => {
                        tracing::debug!("received start token in chunk");
                        let mut new_line = lines.remove(0).to_string();
                        new_line = new_line[1..new_line.len()].to_string();
                        lines.insert(0, new_line);
                        pending_chunk = String::new();
                        ChunkType::Start
                    }
                    "," => {
                        tracing::debug!("received continue token in chunk");
                        lines.remove(0);
                        pending_chunk = String::new();
                        ChunkType::Comma
                    }
                    "]" => {
                        tracing::debug!("received end token in chunk");
                        ChunkType::End
                    }
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
                    if let Err(parse_error) = serde_json::from_str::<Value>(content.as_str()) {
                        tracing::debug!(error=?parse_error, "could not parse chunk as json, setting as pending chunk");
                        pending_chunk.push_str(content.as_str());
                        continue;
                    } else {
                        pending_chunk = String::new();
                    }
                }

                let res = match serde_json::from_str::<GenerateContentResponse>(&content) {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!(error=?e, "failed to parse response from google vertex");
                        if let Err(send_error) =
                            wx.send(Err(GeminiError::ParseError(e.to_string())))
                        {
                            tracing::error!(error=?send_error, "failed to send error message to stream");
                            break;
                        }
                        return;
                    }
                };

                if let Err(send_error) = wx.send(Ok(res)) {
                    tracing::error!(error=?send_error, "failed to send response to stream");
                    break;
                }
            }
        });

        Ok(Box::pin(
            tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
        ))
    }
}

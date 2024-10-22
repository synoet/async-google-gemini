use std::pin::Pin;

use crate::{client::Client, error::GeminiError, types::content::GenerateContentErrorResponse};
use futures::{stream::StreamExt, Stream};
use reqwest_eventsource::{Event, RequestBuilderExt};
use tokio::sync::mpsc;

use crate::types::{
    content::{GenerateContentRequest, GenerateContentResponse},
    gemini::GeminiModel,
};

pub struct Gemini<'c> {
    client: &'c Client,
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
    pub async fn stream_generate_content(
        &self,
        model: GeminiModel,
        request: GenerateContentRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<GenerateContentResponse, GeminiError>> + Send + 'static>>,
        GeminiError,
    > {
        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent?alt=sse",
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
            let mut source = match client
                .post(&url)
                .header("content-type", "application/json; charset=utf-8")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .eventsource()
            {
                Ok(res) => res,

                Err(e) => {
                    tracing::error!(error=?e, "failed to send request to google vertex");
                    return;
                }
            };

            while let Some(sse_event) = source.next().await {
                let event = match sse_event {
                    Ok(e) => match e {
                        Event::Message(m) => m,
                        Event::Open => {
                            tracing::trace!("received open event");
                            continue;
                        }
                    },
                    Err(e) => {
                        tracing::error!(error=?e, "failed to read chunk from google vertex");
                        return;
                    }
                };

                let message = event.data;

                let res = match serde_json::from_str::<GenerateContentResponse>(&message) {
                    Ok(c) => c,
                    Err(parse_error) => {
                        // Check if the message is an error message
                        match serde_json::from_str::<GenerateContentErrorResponse>(&message) {
                            Ok(c) => {
                                tracing::error!(error=?c, "generate content failed");
                                if let Err(send_error) = wx.send(Err(c.into())) {
                                    tracing::error!(
                                        error=?send_error,
                                        "failed to send error message to stream"
                                    );
                                    break;
                                }
                                return;
                            }
                            // this is not an error message
                            Err(_) => {
                                tracing::error!(error=?parse_error, "failed to parse response from google vertex");
                                if let Err(send_error) =
                                    wx.send(Err(GeminiError::ParseError(parse_error.to_string())))
                                {
                                    tracing::error!(error=?send_error, "failed to send error message to stream");
                                    break;
                                }
                                return;
                            }
                        };
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

use std::pin::Pin;

use crate::{
    client::Client,
    error::ClaudeError,
    types::claude::{
        ClaudeModel, RawPredictErrorResponse, RawPredictRequest, RawPredictResponse,
        StreamRawPredictResponse,
    },
};
use futures::{stream::StreamExt, Stream};
use reqwest_eventsource::{Event, RequestBuilderExt};
use tokio::sync::mpsc;

pub struct Claude<'c> {
    client: &'c Client,
}

impl<'c> Claude<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Creates a chat response
    pub async fn raw_predict(
        &self,
        model: ClaudeModel,
        request: RawPredictRequest,
    ) -> Result<RawPredictResponse, ClaudeError> {
        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/anthropic/models/{}:streamRawPredict",
            "us-east5".to_string(),
            self.client.config.project_id(),
            "us-east5".to_string(),
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
                tracing::error!(error=?e, "failed to send request to anthropic");
                return Err(e.into());
            }
        };

        let json = res.json::<serde_json::Value>().await.map_err(|e| {
            tracing::error!(error=?e, "failed to parse response from anthropic");
            ClaudeError::ParseError(e.to_string())
        })?;

        let response = serde_json::from_value::<RawPredictResponse>(json)
            .map_err(|e| ClaudeError::ParseError(format!("failed to parse response: {}", e)))?;

        Ok(response)
    }
    /// Create a chat stream response
    /// partial message deltas will be sent as stream chunks
    pub async fn stream_raw_predict(
        &self,
        model: ClaudeModel,
        request: RawPredictRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<StreamRawPredictResponse, ClaudeError>> + Send + 'static>>,
        ClaudeError,
    > {
        let url = format!("https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/anthropic/models/{}:streamRawPredict?alt=sse",
            "us-east5".to_string(),
            self.client.config.project_id(),
            "us-east5".to_string(),
            model.to_string(),
        );

        let client = self.client.http_client.clone();

        let token = self.client.config.token().await.map_err(|e| {
            tracing::error!(error=?e, "failed to get authentication token");
            ClaudeError::AuthenticationError(e.to_string())
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
                    tracing::error!(error=?e, "failed to send request to anthropic");
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
                        tracing::error!(error=?e, "failed to send request to anthropic");
                        return;
                    }
                };

                let message = event.data;

                let res = match serde_json::from_str::<StreamRawPredictResponse>(&message) {
                    Ok(c) => c,
                    Err(parse_error) => {
                        match serde_json::from_str::<RawPredictErrorResponse>(&message) {
                            Ok(c) => {
                                tracing::error!(error=?c, "stream raw predict failed");
                                if let Err(send_error) = wx.send(Err(c.into())) {
                                    tracing::error!(
                                        error=?send_error,
                                        "failed to send error message to stream"
                                    );
                                    break;
                                }
                                return;
                            }
                            Err(_) => {
                                tracing::error!(error=?parse_error, "failed to parse response from google vertex");
                                if let Err(send_error) =
                                    wx.send(Err(ClaudeError::ParseError(parse_error.to_string())))
                                {
                                    tracing::error!(error=?send_error, "failed to send error message to stream");
                                    break;
                                }
                                return;
                            }
                        }
                    }
                };

                let is_stop = match res {
                    StreamRawPredictResponse::MessageStop => true,
                    _ => false,
                };

                if let Err(send_error) = wx.send(Ok(res)) {
                    tracing::error!(error=?send_error, "failed to send response to stream");
                    break;
                }

                if is_stop {
                    break;
                }
            }
        });

        Ok(Box::pin(
            tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
        ))
    }
}

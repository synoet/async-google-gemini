use crate::{chat::Chat, config::GeminiConfig, error::ClientError};

pub struct GeminiClient {
    pub http_client: reqwest::Client,
    pub config: GeminiConfig,
}

impl GeminiClient {
    pub fn new() -> Result<Self, ClientError> {
        let config = GeminiConfig::try_from_env()?;
        Ok(Self {
            http_client: reqwest::Client::new(),
            config,
        })
    }

    pub fn chat(&self) -> Chat {
        Chat::new(self)
    }
}

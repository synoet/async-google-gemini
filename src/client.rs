use crate::{claude::Claude, config::GeminiConfig, error::ClientError, gemini::Gemini};

pub struct Client {
    pub http_client: reqwest::Client,
    pub config: GeminiConfig,
}

impl Client {
    pub fn new(config: GeminiConfig) -> Result<Self, ClientError> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            config,
        })
    }

    pub fn gemini(&self) -> Gemini {
        Gemini::new(self)
    }

    pub fn claude(&self) -> Claude {
        Claude::new(self)
    }
}

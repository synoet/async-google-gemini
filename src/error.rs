#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Missing Environment Variable {0}")]
    MissingEnvVar(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GeminiError {
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Failed to send request: {0}")]
    RequestError(#[from] reqwest::Error),
}

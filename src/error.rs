#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Missing Environment Variable {0}")]
    MissingEnvVar(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GeminiError {
    #[error("Request fails API validation, or you tried to access a model that requires allowlisting or is disallowed by the organization's policy.")]
    InvalidArgument,
    #[error("Client doesn't have sufficient permission to call the API.")]
    PermissionDenied,
    #[error("No valid object is found from the designated URL.")]
    NotFound,
    #[error("RESOURCE_EXHAUSTED")]
    ResourceExhausted,
    #[error("Request is cancelled by the client.")]
    Cancelled,
    #[error("Request is not valid.")]
    Internal,
    #[error("Service is temporarily unavailable.")]
    Unavailable,
    #[error("EXCEEDED	The client sets a deadline shorter than the server's default deadline (10 minutes), and the request didn't finish within the client-provided deadline.")]
    DeadlineExceeded,
    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

impl From<reqwest::Error> for GeminiError {
    fn from(e: reqwest::Error) -> Self {
        let mut error = Self::Internal;
        if let Some(e) = e.status() {
            error = match e.as_u16() {
                400 => Self::InvalidArgument,
                403 => Self::PermissionDenied,
                404 => Self::NotFound,
                429 => Self::ResourceExhausted,
                499 => Self::Cancelled,
                500 => Self::Internal,
                503 => Self::Unavailable,
                504 => Self::DeadlineExceeded,
                _ => Self::Internal,
            };
        }

        error
    }
}

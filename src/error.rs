use crate::types::{claude::RawPredictErrorResponse, content::GenerateContentErrorResponse};
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
    #[error("Failed to generate authentication token {0}")]
    AuthenticationError(String),
}

impl From<usize> for GeminiError {
    fn from(u: usize) -> Self {
        match u {
            400 => Self::InvalidArgument,
            403 => Self::PermissionDenied,
            404 => Self::NotFound,
            429 => Self::ResourceExhausted,
            499 => Self::Cancelled,
            500 => Self::Internal,
            503 => Self::Unavailable,
            504 => Self::DeadlineExceeded,
            _ => Self::Internal,
        }
    }
}

impl From<reqwest::Error> for GeminiError {
    fn from(e: reqwest::Error) -> Self {
        let mut error = Self::Internal;
        if let Some(e) = e.status() {
            error = GeminiError::from(e.as_u16() as usize)
        }

        error
    }
}

impl From<GenerateContentErrorResponse> for GeminiError {
    fn from(e: GenerateContentErrorResponse) -> Self {
        GeminiError::from(e.error.code)
    }
}

#[derive(Debug, thiserror::Error, strum_macros::EnumString)]
pub enum ClaudeError {
    #[error("There was an issue with the format or content of your request {0}")]
    #[strum(serialize = "invalid_request_error")]
    InvalidRequestError(String),
    #[error("There’s an issue with your API key. {0}")]
    #[strum(serialize = "authentication_error")]
    AuthenticationError(String),
    #[error("Your API key does not have permission to use the specified resource. {0}")]
    #[strum(serialize = "permission_error")]
    PermissionError(String),
    #[error("The requested resource was not found. {0}")]
    #[strum(serialize = "not_found_error")]
    NotFoundError(String),
    #[error("Request exceeds the maximum allowed number of bytes. {0}")]
    #[strum(serialize = "request_too_large")]
    RequestTooLarge(String),
    #[error("Your account has hit a rate limit. {0}")]
    #[strum(serialize = "rate_limit_error")]
    RateLimitError(String),
    #[error("An unexpected error has occurred internal to Anthropic’s systems. {0}")]
    #[strum(serialize = "api_error")]
    ApiError(String),
    #[error("Anthropic’s API is temporarily overloaded. {0}")]
    #[strum(serialize = "overloaded_error")]
    OverloadedError(String),
    #[error("Internal error {0}")]
    #[strum(serialize = "internal_error")]
    Internal(String),
    #[error("failed to parse response from anthropic {0}")]
    #[strum(serialize = "parse_error")]
    ParseError(String),
}

impl From<RawPredictErrorResponse> for ClaudeError {
    fn from(e: RawPredictErrorResponse) -> Self {
        match e.e_type.as_str() {
            "invalid_request_error" => ClaudeError::InvalidRequestError(e.error.message),
            "authentication_error" => ClaudeError::AuthenticationError(e.error.message),
            "permission_error" => ClaudeError::PermissionError(e.error.message),
            "not_found_error" => ClaudeError::NotFoundError(e.error.message),
            "request_too_large" => ClaudeError::RequestTooLarge(e.error.message),
            "rate_limit_error" => ClaudeError::RateLimitError(e.error.message),
            "api_error" => ClaudeError::ApiError(e.error.message),
            "overloaded_error" => ClaudeError::OverloadedError(e.error.message),
            _ => ClaudeError::Internal(e.error.message),
        }
    }
}

impl From<reqwest::Error> for ClaudeError {
    fn from(e: reqwest::Error) -> Self {
        ClaudeError::Internal(e.to_string())
    }
}

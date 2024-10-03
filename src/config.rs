use crate::error::ClientError;

#[derive(Debug)]
pub struct GeminiConfig {
    pub location: String,
    pub api_key: String,
    pub project_id: String,
}

impl GeminiConfig {
    pub fn new(location: String, api_key: String, project_id: String) -> Self {
        Self {
            location,
            api_key,
            project_id,
        }
    }
    pub fn try_from_env() -> Result<Self, ClientError> {
        Ok(Self {
            location: std::env::var("GEMINI_LOCATION").unwrap_or("us-central1".to_string()),
            api_key: std::env::var("GEMINI_API_KEY")
                .map_err(|_| ClientError::MissingEnvVar("GEMINI_API_KEY".to_string()))?,
            project_id: std::env::var("GEMINI_PROJECT_ID")
                .map_err(|_| ClientError::MissingEnvVar("GEMINI_PROJECT_ID".to_string()))?,
        })
    }
}

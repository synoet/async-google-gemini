use anyhow::{anyhow, Result};
use std::sync::Arc;

use gcp_auth::{CustomServiceAccount, TokenProvider};

use crate::error::ClientError;

#[derive(Debug, Clone)]
pub enum ConfigSource {
    ServiceAccount { account: Arc<CustomServiceAccount> },
    Environment { token: String },
}

const SCOPES: &[&str] = &["https://www.googleapis.com/auth/cloud-platform"];

#[derive(Debug)]
pub struct GeminiConfig {
    config_source: ConfigSource,
    location: String,
    project_id: String,
}

impl GeminiConfig {
    /// Attempts to load the [GeminiConfig] from environment variables.
    /// This is useful for running the client locally
    /// Running in production, you should use the [GeminiConfig::try_from_service_account_file()] method instead.
    ///
    /// # Variables:
    /// - `GEMINI_LOCATION`: The location of the Gemini endpoint.
    /// - `GEMINI_TOKEN`: The API token for the Gemini endpoint.
    /// - `GEMINI_PROJECT_ID`: The project ID for the Gemini endpoint.
    pub fn try_from_env_vars() -> Result<Self, ClientError> {
        Ok(Self {
            config_source: ConfigSource::Environment {
                token: std::env::var("GCP_TOKEN")
                    .map_err(|_| ClientError::MissingEnvVar("GCP_TOKEN".to_string()))?,
            },
            location: std::env::var("GCP_LOCATION").unwrap_or("us-central1".to_string()),
            project_id: std::env::var("GCP_PROJECT_ID")
                .map_err(|_| ClientError::MissingEnvVar("GCP_PROJECT_ID".to_string()))?,
        })
    }

    /// Attemps to load the [GeminiConfig] from a service account json file.
    ///
    /// # Arguments:
    /// - `file_path`: The path to the service account json file.
    pub fn try_from_service_account_file(file_path: String) -> Result<Self> {
        let contents = std::fs::read_to_string(file_path)?;
        let account = CustomServiceAccount::from_json(contents.as_str())?;

        let project_id = match account.project_id() {
            Some(project_id) => project_id.to_string(),
            None => return Err(anyhow!("Service Account does not have a project ID")),
        };

        Ok(Self {
            config_source: ConfigSource::ServiceAccount {
                account: Arc::new(account),
            },
            location: std::env::var("GCP_LOCATION").unwrap_or("us-central1".to_string()),
            project_id,
        })
    }

    /// Attemps to load the [GeminiConfig] from a service account json in a ["GOOGLE_SERVICE_ACCOUNT"] environment variable.
    pub fn try_from_service_account_env() -> Result<Self> {
        let content = std::env::var("GCP_SERVICE_ACCOUNT")?;
        let account = CustomServiceAccount::from_json(content.as_str())?;

        let project_id = match account.project_id() {
            Some(project_id) => project_id.to_string(),
            None => return Err(anyhow!("Service Account does not have a project ID")),
        };

        Ok(Self {
            config_source: ConfigSource::ServiceAccount {
                account: Arc::new(account),
            },
            location: std::env::var("GCP_LOCATION").unwrap_or("us-central1".to_string()),
            project_id,
        })
    }

    pub fn location(&self) -> &str {
        self.location.as_str()
    }

    pub fn project_id(&self) -> &str {
        self.project_id.as_str()
    }

    pub async fn token(&self) -> Result<String> {
        match &self.config_source {
            ConfigSource::ServiceAccount { account } => {
                let token = account.token(&SCOPES).await?;
                Ok(token.as_str().to_string())
            }
            ConfigSource::Environment { token, .. } => Ok(token.to_owned()),
        }
    }
}

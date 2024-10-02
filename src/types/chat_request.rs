use crate::types::model::GeminiModel;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PartData {
    Text(String),
    InlineData { mime_type: String, data: String },
    FileData { mime_type: String, file_uri: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VideoMetadata {
    pub start_offset: TimeOffset,
    pub end_offset: TimeOffset,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TimeOffset {
    pub seconds: i64,
    pub nanos: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Part {
    #[serde(flatten)]
    pub data: PartData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SystemInstruction {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Tool {
    pub function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
struct Setting {
    category: String,
    threshold: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SafetySetting(Vec<Setting>);

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<serde_json::Value>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_logprobs: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_timestamp: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, PartialEq)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct GeminiChatRequest {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
    pub contents: Vec<Content>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<SafetySetting>,
    #[builder(default)]
    pub generation_config: Option<GenerationConfig>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[builder(default)]
    #[serde(skip_serializing)]
    pub model: Option<GeminiModel>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            temperature: Some(1.0),
            max_output_tokens: Some(1024),
            top_p: Some(0.95),
            top_k: None,
            candidate_count: None,
            presence_penalty: None,
            frequency_penalty: None,
            stop_sequences: None,
            response_mime_type: None,
            response_schema: None,
            seed: None,
            response_logprobs: None,
            logprobs: None,
            audio_timestamp: None,
        }
    }
}

impl Default for SafetySetting {
    fn default() -> Self {
        Self {
            0: vec![
                Setting {
                    category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                    threshold: "OFF".to_string(),
                },
                Setting {
                    category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                    threshold: "OFF".to_string(),
                },
                Setting {
                    category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                    threshold: "OFF".to_string(),
                },
                Setting {
                    category: "HARM_CATEGORY_HARASSMENT".to_string(),
                    threshold: "OFF".to_string(),
                },
            ],
        }
    }
}

impl GeminiChatRequest {
    pub fn builder() -> GeminiChatRequestBuilder {
        GeminiChatRequestBuilder::default()
    }
}

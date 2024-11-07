use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

#[derive(
    Serialize,
    Deserialize,
    strum_macros::Display,
    IntoStaticStr,
    strum_macros::EnumString,
    Clone,
    Debug,
)]
pub enum ClaudeModel {
    #[serde(rename = "claude-3-5-sonnet-v2@20241022")]
    #[strum(serialize = "claude-3-5-sonnet-v2@20241022")]
    Claude35SonnetV2,
    #[serde(rename = "claude-3-5-sonnet@20230725")]
    #[strum(serialize = "claude-3-5-sonnet@20230725")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-opus@20230725")]
    #[strum(serialize = "claude-3-opus@20230725")]
    Claude3Opus,
    #[serde(rename = "claude-3-haiku@20240307")]
    #[strum(serialize = "claude-3-haiku@20240307")]
    Claude3Haiku,
    #[serde(rename = "claude-3-sonnet@20240229")]
    #[strum(serialize = "claude-3-sonnet@20240229")]
    Claude3Sonnet,
}

pub trait WithoutVersion {
    fn without_version(&self) -> String;
    fn from_without_version(s: String) -> Self;
}

impl WithoutVersion for ClaudeModel {
    fn without_version(&self) -> String {
        match self {
            ClaudeModel::Claude35SonnetV2 => "claude-3.5-sonnet-v2".to_string(),
            ClaudeModel::Claude35Sonnet => "claude-3.5-sonnet".to_string(),
            ClaudeModel::Claude3Opus => "claude-3-opus".to_string(),
            ClaudeModel::Claude3Haiku => "claude-3-haiku".to_string(),
            ClaudeModel::Claude3Sonnet => "claude-3-sonnet".to_string(),
        }
    }

    fn from_without_version(s: String) -> Self {
        match s.as_str() {
            "claude-3.5-sonnet-v2" => ClaudeModel::Claude35SonnetV2,
            "claude-3.5-sonnet" => ClaudeModel::Claude35Sonnet,
            "claude-3-opus" => ClaudeModel::Claude3Opus,
            "claude-3-haiku" => ClaudeModel::Claude3Haiku,
            "claude-3-sonnet" => ClaudeModel::Claude3Sonnet,
            _ => ClaudeModel::Claude35SonnetV2,
        }
    }
}

const ANTHROPIC_VERSION: &str = "vertex-2023-10-16";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ClaudeSystemPrompt {
    Text(String),
    Content(Vec<ClaudeContent>),
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug)]
pub struct RawPredictRequest {
    #[builder(default = "ANTHROPIC_VERSION.to_string()")]
    pub anthropic_version: String,
    pub max_tokens: u32,
    pub system: ClaudeSystemPrompt,
    pub stream: bool,
    pub messages: Vec<ClaudeMessage>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub c_type: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClaudeStopReason {
    #[serde(rename = "end_turn")]
    EndTurn,
    #[serde(rename = "max_tokens")]
    MaxTokens,
    #[serde(rename = "max_time")]
    StopSequence,
    #[serde(rename = "tool_use")]
    ToolUse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClaudeUsage {
    /// The number of input tokens which were used.
    pub input_tokens: u32,
    /// The number of output tokens which were used.
    pub output_tokens: u32,
}

#[derive(Serialize, Deserialize, Builder, Clone, Debug)]
pub struct RawPredictResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub m_type: String,
    pub role: String,
    pub content: Vec<ClaudeContent>,
    pub stop_reason: Option<ClaudeStopReason>,
    pub stop_sequence: Option<String>,
    pub usage: ClaudeUsage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageDelta {
    stop_reason: Option<ClaudeStopReason>,
    stop_sequence: Option<String>,
    usage: Option<ClaudeUsage>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StreamRawPredictResponse {
    #[serde(rename = "message_start")]
    MessageStart { message: RawPredictResponse },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: Option<u32>,
        content_block: ClaudeContent,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: Option<u32>,
        delta: ClaudeContent,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: Option<u32> },
    #[serde(rename = "message_delta")]
    MessageDelta {
        index: Option<u32>,
        delta: MessageDelta,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
}

impl RawPredictRequest {
    pub fn builder() -> RawPredictRequestBuilder {
        RawPredictRequestBuilder::default()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClaudeError {
    pub e_type: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawPredictErrorResponse {
    #[serde(rename = "type")]
    pub e_type: String,
    pub error: ClaudeError,
}

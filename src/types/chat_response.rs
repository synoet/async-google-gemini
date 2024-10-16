use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GeminiChatResponse {
    pub candidates: Option<Vec<Candidate>>,
    pub usage_metadata: Option<UsageMetadata>,
    pub model_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: Option<FinishReason>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_logprobs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs_result: Option<LogprobsResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
    pub blocked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    Harassment,
    HateSpeech,
    SexuallyExplicit,
    DangerousContent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CitationMetadata {
    pub citations: Vec<Citation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Citation {
    pub start_index: i32,
    pub end_index: i32,
    pub uri: String,
    pub title: String,
    pub license: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogprobsResult {
    pub top_candidates: Vec<TopCandidate>,
    pub chosen_candidates: Vec<ChosenCandidate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TopCandidate {
    pub candidates: Vec<TokenProbability>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChosenCandidate {
    pub token: String,
    pub log_probability: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokenProbability {
    pub token: String,
    pub log_probability: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: i32,
    pub candidates_token_count: i32,
    pub total_token_count: i32,
}

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Assume GoogleAuthOptions is defined elsewhere
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GoogleAuthOptions {
    // Define fields as per your requirements
}

// Enums for Harm Categories
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    HarmCategoryUnspecified,
    HarmCategoryHateSpeech,
    HarmCategoryDangerousContent,
    HarmCategoryHarassment,
    HarmCategorySexuallyExplicit,
}

// Enums for Harm Block Threshold
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    HarmBlockThresholdUnspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
}

// Enums for Harm Probability
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    HarmProbabilityUnspecified,
    Negligible,
    Low,
    Medium,
    High,
}

// Enums for Harm Severity
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmSeverity {
    HarmSeverityUnspecified,
    HarmSeverityNegligible,
    HarmSeverityLow,
    HarmSeverityMedium,
    HarmSeverityHigh,
}

// Enums for Blocked Reason
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockedReason {
    BlockedReasonUnspecified,
    Safety,
    Other,
    Blocklist,
    ProhibitedContent,
}

// Enums for Finish Reason
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {
    FinishReasonUnspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
    Blocklist,
    ProhibitedContent,
    Spii,
}

// VertexInit struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct VertexInit {
    /// Optional. The Google Cloud project ID.
    pub project: Option<String>,
    /// Optional. The Google Cloud project location.
    pub location: Option<String>,
    /// Optional. The base Vertex AI endpoint to use for the request.
    #[serde(rename = "apiEndpoint")]
    pub api_endpoint: Option<String>,
    /// Optional. The Authentication options provided by google-auth-library.
    #[serde(rename = "googleAuthOptions")]
    pub google_auth_options: Option<GoogleAuthOptions>,
}

// BaseModelParams struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct BaseModelParams {
    /// Optional. Array of SafetySetting.
    #[serde(rename = "safetySettings", skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    /// Optional. GenerationConfig.
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Optional. Array of Tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Optional. This config is shared for all tools provided in the request.
    #[serde(rename = "toolConfig", skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    /// Optional. The user provided system instructions for the model.
    #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
}

// SystemInstruction enum
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SystemInstruction {
    Text(String),
    Content(Content),
}

// GenerateContentRequest struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GenerateContentRequest {
    #[serde(flatten)]
    pub base_model_params: BaseModelParams,
    /// Array of Content.
    pub contents: Vec<Content>,
    /// Optional. The name of the cached content used as context.
    #[serde(rename = "cachedContent", skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
}

impl GenerateContentRequest {
    pub fn builder() -> GenerateContentRequestBuilder {
        GenerateContentRequestBuilder::default()
    }
}

// Content struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

// Part enum
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Part {
    TextPart(TextPart),
    InlineDataPart(InlineDataPart),
    FileDataPart(FileDataPart),
    FunctionResponsePart(FunctionResponsePart),
    FunctionCallPart(FunctionCallPart),
}

// TextPart struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TextPart {
    pub text: String,
}

// InlineDataPart struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InlineDataPart {
    #[serde(rename = "inlineData")]
    pub inline_data: GenerativeContentBlob,
}

// GenerativeContentBlob struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GenerativeContentBlob {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub data: String,
}

// FileDataPart struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileDataPart {
    #[serde(rename = "fileData")]
    pub file_data: FileData,
}

// FileData struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FileData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "fileUri")]
    pub file_uri: String,
}

// FunctionResponsePart struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionResponsePart {
    #[serde(rename = "functionResponse")]
    pub function_response: FunctionResponse,
}

// FunctionCallPart struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionCallPart {
    #[serde(rename = "functionCall")]
    pub function_call: FunctionCall,
}

// FunctionResponse struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionResponse {
    pub name: String,
    pub response: Value, // Assuming dynamic JSON object
}

// FunctionCall struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Value, // Assuming dynamic JSON object
}

// SafetySetting struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SafetySetting {
    pub category: Option<HarmCategory>,
    pub threshold: Option<HarmBlockThreshold>,
}

// GenerationConfig struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct GenerationConfig {
    #[serde(rename = "candidateCount", skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<u32>,
    #[serde(rename = "stopSequences", skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(rename = "maxOutputTokens", skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(rename = "topP", skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(rename = "topK", skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(rename = "frequencyPenalty", skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(rename = "responseMimeType", skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,
    #[serde(rename = "responseSchema", skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<ResponseSchema>,
}

// ResponseSchema struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ResponseSchema {
    // Define fields as needed
}

// Tool enum
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Tool {
    FunctionDeclarationsTool(FunctionDeclarationsTool),
    RetrievalTool(RetrievalTool),
    GoogleSearchRetrievalTool(GoogleSearchRetrievalTool),
}

// FunctionDeclarationsTool struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionDeclarationsTool {
    #[serde(rename = "functionDeclarations")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,
}

// FunctionDeclaration struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<FunctionDeclarationSchema>,
}

// FunctionDeclarationSchema struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FunctionDeclarationSchema {
    #[serde(rename = "type")]
    pub schema_type: SchemaType,
    pub properties: HashMap<String, FunctionDeclarationSchemaProperty>,
    pub description: Option<String>,
    pub required: Option<Vec<String>>,
}

// SchemaType enum
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum SchemaType {
    Object,
    String,
    Number,
    Integer,
    Boolean,
    Array,
}

// FunctionDeclarationSchemaProperty type
pub type FunctionDeclarationSchemaProperty = Schema;

// Schema struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<SchemaType>,
    pub properties: Option<HashMap<String, Schema>>,
    pub description: Option<String>,
    pub required: Option<Vec<String>>,
    // Add other fields as needed
}

// RetrievalTool struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RetrievalTool {
    pub retrieval: Option<Retrieval>,
}

// Retrieval struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Retrieval {
    #[serde(rename = "vertexAiSearch")]
    pub vertex_ai_search: Option<VertexAISearch>,
    #[serde(rename = "vertexRagStore")]
    pub vertex_rag_store: Option<VertexRagStore>,
    #[serde(rename = "disableAttribution", skip_serializing_if = "Option::is_none")]
    pub disable_attribution: Option<bool>,
}

// VertexAISearch struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VertexAISearch {
    pub datastore: String,
}

// VertexRagStore struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VertexRagStore {
    #[serde(rename = "ragResources", skip_serializing_if = "Option::is_none")]
    pub rag_resources: Option<Vec<RagResource>>,
    #[serde(rename = "similarityTopK", skip_serializing_if = "Option::is_none")]
    pub similarity_top_k: Option<u32>,
    #[serde(
        rename = "vectorDistanceThreshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub vector_distance_threshold: Option<f32>,
}

// RagResource struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RagResource {
    #[serde(rename = "ragCorpus", skip_serializing_if = "Option::is_none")]
    pub rag_corpus: Option<String>,
    #[serde(rename = "ragFileIds", skip_serializing_if = "Option::is_none")]
    pub rag_file_ids: Option<Vec<String>>,
}

// GoogleSearchRetrievalTool struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GoogleSearchRetrievalTool {
    #[serde(rename = "googleSearchRetrieval")]
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,
}

// GoogleSearchRetrieval struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GoogleSearchRetrieval {
    // Define fields if any
}

// ToolConfig struct
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolConfig {
    // Define fields as needed
}

// SafetyRating struct
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct SafetyRating {
    pub category: Option<HarmCategory>,
    pub probability: Option<HarmProbability>,
    #[serde(rename = "probabilityScore", skip_serializing_if = "Option::is_none")]
    pub probability_score: Option<f32>,
    pub severity: Option<HarmSeverity>,
    #[serde(rename = "severityScore", skip_serializing_if = "Option::is_none")]
    pub severity_score: Option<f32>,
}

// UsageMetadata struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount", skip_serializing_if = "Option::is_none")]
    pub prompt_token_count: Option<u32>,
    #[serde(
        rename = "candidatesTokenCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub candidates_token_count: Option<u32>,
    #[serde(rename = "totalTokenCount", skip_serializing_if = "Option::is_none")]
    pub total_token_count: Option<u32>,
    #[serde(
        rename = "cachedContentTokenCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_content_token_count: Option<u32>,
}

// PromptFeedback struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PromptFeedback {
    #[serde(rename = "blockReason")]
    pub block_reason: Option<BlockedReason>,
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Vec<SafetyRating>,
    #[serde(rename = "blockReasonMessage", skip_serializing_if = "Option::is_none")]
    pub block_reason_message: Option<String>,
}

// GenerateContentResponse struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GenerateContentResponse {
    pub candidates: Option<Vec<GenerateContentCandidate>>,
    #[serde(rename = "promptFeedback")]
    pub prompt_feedback: Option<PromptFeedback>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,
}

// GenerateContentCandidate struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GenerateContentCandidate {
    pub content: Content,
    pub index: Option<u32>,
    #[serde(rename = "finishReason", skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<FinishReason>,
    #[serde(rename = "finishMessage", skip_serializing_if = "Option::is_none")]
    pub finish_message: Option<String>,
    #[serde(rename = "safetyRatings", skip_serializing_if = "Option::is_none")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(rename = "citationMetadata", skip_serializing_if = "Option::is_none")]
    pub citation_metadata: Option<CitationMetadata>,
    #[serde(rename = "groundingMetadata", skip_serializing_if = "Option::is_none")]
    pub grounding_metadata: Option<GroundingMetadata>,
}

// CitationMetadata struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CitationMetadata {
    pub citations: Vec<Citation>,
}

// Citation struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Citation {
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<u32>,
    pub uri: Option<String>,
    pub title: Option<String>,
    pub license: Option<String>,
    #[serde(rename = "publicationDate", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<GoogleDate>,
}

// GoogleDate struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GoogleDate {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

// GroundingMetadata struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GroundingMetadata {
    #[serde(rename = "webSearchQueries", skip_serializing_if = "Option::is_none")]
    pub web_search_queries: Option<Vec<String>>,
    #[serde(rename = "retrievalQueries", skip_serializing_if = "Option::is_none")]
    pub retrieval_queries: Option<Vec<String>>,
    #[serde(rename = "searchEntryPoint", skip_serializing_if = "Option::is_none")]
    pub search_entry_point: Option<SearchEntryPoint>,
    #[serde(rename = "groundingChunks", skip_serializing_if = "Option::is_none")]
    pub grounding_chunks: Option<Vec<GroundingChunk>>,
    #[serde(rename = "groundingSupports", skip_serializing_if = "Option::is_none")]
    pub grounding_supports: Option<Vec<GroundingSupport>>,
}

// SearchEntryPoint struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SearchEntryPoint {
    #[serde(rename = "renderedContent", skip_serializing_if = "Option::is_none")]
    pub rendered_content: Option<String>,
    #[serde(rename = "sdkBlob", skip_serializing_if = "Option::is_none")]
    pub sdk_blob: Option<String>,
}

// GroundingChunk struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GroundingChunk {
    pub web: Option<GroundingChunkWeb>,
    #[serde(rename = "retrievedContext")]
    pub retrieved_context: Option<GroundingChunkRetrievedContext>,
}

// GroundingChunkWeb struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GroundingChunkWeb {
    pub uri: Option<String>,
    pub title: Option<String>,
}

// GroundingChunkRetrievedContext struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GroundingChunkRetrievedContext {
    pub uri: Option<String>,
    pub title: Option<String>,
}

// GroundingSupport struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GroundingSupport {
    pub segment: Option<GroundingSupportSegment>,
    #[serde(
        rename = "groundingChunkIndices",
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_chunk_indices: Option<Vec<u32>>,
    #[serde(rename = "confidenceScores", skip_serializing_if = "Option::is_none")]
    pub confidence_scores: Option<Vec<f32>>,
}

// GroundingSupportSegment struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GroundingSupportSegment {
    #[serde(rename = "partIndex", skip_serializing_if = "Option::is_none")]
    pub part_index: Option<u32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(rename = "endIndex", skip_serializing_if = "Option::is_none")]
    pub end_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

// StartChatParams struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct StartChatParams {
    pub history: Option<Vec<Content>>,
    #[serde(rename = "safetySettings", skip_serializing_if = "Option::is_none")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(rename = "toolConfig", skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    #[serde(rename = "apiEndpoint", skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
    #[serde(rename = "cachedContent", skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
}

// StartChatSessionRequest struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct StartChatSessionRequest {
    #[serde(flatten)]
    pub start_chat_params: StartChatParams,
    pub project: String,
    pub location: String,
    #[serde(rename = "googleAuth")]
    pub google_auth: GoogleAuth,
    #[serde(rename = "publisherModelEndpoint")]
    pub publisher_model_endpoint: String,
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
}

// GoogleAuth struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GoogleAuth {
    // Define fields as per your requirements
}

// RequestOptions struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RequestOptions {
    pub timeout: Option<u64>,
    #[serde(rename = "apiClient", skip_serializing_if = "Option::is_none")]
    pub api_client: Option<String>,
    #[serde(rename = "customHeaders", skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<HashMap<String, String>>,
}

// CachedContent struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CachedContent {
    pub name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "systemInstruction", skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<SystemInstruction>,
    pub contents: Option<Vec<Content>>,
    pub tools: Option<Vec<Tool>>,
    #[serde(rename = "toolConfig", skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename = "usageMetadata", skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<CachedContentUsageMetadata>,
    #[serde(rename = "expireTime", skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    pub ttl: Option<String>,
}

// CachedContentUsageMetadata struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct CachedContentUsageMetadata {
    #[serde(rename = "totalTokenCount", skip_serializing_if = "Option::is_none")]
    pub total_token_count: Option<u32>,
    #[serde(rename = "textCount", skip_serializing_if = "Option::is_none")]
    pub text_count: Option<u32>,
    #[serde(rename = "imageCount", skip_serializing_if = "Option::is_none")]
    pub image_count: Option<u32>,
    #[serde(
        rename = "videoDurationSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub video_duration_seconds: Option<u32>,
    #[serde(
        rename = "audioDurationSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_duration_seconds: Option<u32>,
}

// ListCachedContentsResponse struct
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ListCachedContentsResponse {
    #[serde(rename = "cachedContents", skip_serializing_if = "Option::is_none")]
    pub cached_contents: Option<Vec<CachedContent>>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

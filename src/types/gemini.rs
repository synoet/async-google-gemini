use serde::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

/// Enum representing different Gemini models with their version identifiers.
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    strum_macros::Display,
    IntoStaticStr,
    strum_macros::EnumString,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum GeminiModel {
    // Gemini 2.0 Flash Lite
    #[serde(rename = "gemini-2.0-flash-lite-001")]
    #[strum(serialize = "gemini-2.0-flash-lite-001")]
    Gemini20FlashLite001,
    // Gemini 2.0 Flash
    #[serde(rename = "gemini-2.0-flash-001")]
    #[strum(serialize = "gemini-2.0-flash-001")]
    Gemini20Flash001,
    // Gemini 1.5 Flash
    #[serde(rename = "gemini-1.5-flash-001")]
    #[strum(serialize = "gemini-1.5-flash-001")]
    Gemini15Flash001,
    #[serde(rename = "gemini-1.5-flash-002")]
    #[strum(serialize = "gemini-1.5-flash-002")]
    Gemini15Flash002,

    #[serde(rename = "gemini-1.5-pro")]
    #[strum(serialize = "gemini-1.5-pro")]
    Gemini15ProLatestStable,
    // Gemini 1.5 Pro
    #[serde(rename = "gemini-1.5-pro-001")]
    #[strum(serialize = "gemini-1.5-pro-001")]
    Gemini15Pro001,

    #[serde(rename = "gemini-1.5-pro-002")]
    #[strum(serialize = "gemini-1.5-pro-002")]
    Gemini15Pro002,

    // Gemini 1.0 Pro Vision
    #[serde(rename = "gemini-1.0-pro-vision-001")]
    #[strum(serialize = "gemini-1.0-pro-vision-001")]
    Gemini10ProVision001,

    // Gemini 1.0 Pro
    #[serde(rename = "gemini-1.0-pro")]
    #[strum(serialize = "gemini-1.0-pro")]
    Gemini10Pro,
    #[serde(rename = "gemini-1.0-pro-001")]
    #[strum(serialize = "gemini-1.0-pro-001")]
    Gemini10Pro001,
    #[serde(rename = "gemini-1.0-pro-002")]
    #[strum(serialize = "gemini-1.0-pro-002")]
    Gemini10Pro002,
}

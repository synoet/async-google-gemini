use serde::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, strum_macros::Display, IntoStaticStr,
)]
pub enum GeminiModel {
    #[strum(serialize = "gemini-1.5-flash-001")]
    Gemini15Flash001,

    #[strum(serialize = "gemini-1.5-pro-001")]
    Gemini15Pro001,

    #[strum(serialize = "gemini-1.5-pro-002")]
    Gemini15Pro002,

    #[strum(serialize = "gemini-1.0-pro-001")]
    Gemini10Pro001,

    #[strum(serialize = "gemini-1.0-pro-vision-001")]
    Gemini10ProVision001,

    #[strum(serialize = "gemini-1.0-pro")]
    Gemini10Pro,

    #[strum(serialize = "gemini-1.0-pro-002")]
    Gemini10Pro002,
}

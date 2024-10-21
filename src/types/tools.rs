use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Function calling mode.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FunctionCallingMode {
    /// Unspecified function calling mode. This value should not be used.
    ModeUnspecified,
    /**
     * Default model behavior, model decides to predict either function calls
     * or natural language response.
     */
    Auto,
    /**
     * Model is constrained to always predicting function calls only.
     * If "allowedFunctionNames" are set, the predicted function calls will be
     * limited to any one of "allowedFunctionNames", else the predicted
     * function calls will be any one of the provided "function_declarations".
     */
    Any,
    /**
     * Model will not predict any function calls. Model behavior is same as when
     * not passing any function declarations.
     */
    None,
}

/// Function calling config.
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct FunctionCallingConfig {
    /// Optional. Function calling mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingMode>,

    /**
     * Optional. Function names to call. Only set when the Mode is ANY. Function
     * names should match [FunctionDeclaration.name]. With mode set to ANY, model
     * will predict a function call from the set of function names provided.
     */
    #[serde(
        rename = "allowedFunctionNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_function_names: Option<Vec<String>>,
}

/// This config is shared for all tools provided in the request.
#[derive(Clone, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct ToolConfig {
    /// Function calling config.
    #[serde(
        rename = "functionCallingConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

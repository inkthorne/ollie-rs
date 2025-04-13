use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ===
// STRUCT: GeminiPartText
// ===

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPartText {
    pub text: String,
}

// ===
// STRUCT: GeminiPartFunctionCall
// ===

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPartFunctionCall {
    pub function_name: String,
    pub arguments: JsonValue,
}

// ===
// STRUCT: GeminiPartUnknown
// ===

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPartUnknown {
    pub value: JsonValue,
}

// ===
// STRUCT: GeminiPart
// ===

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiPart {
    Text(GeminiPartText),
    FunctionCall(GeminiPartFunctionCall),
}

// ===
// STRUCT: GeminiContent1
// ===

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent1 {
    pub role: Option<String>,
    pub parts: Vec<GeminiPart>,
}

// ===
// STRUCT: GeminiCandidate
// ===

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiCandidate {
    pub index: Option<u32>,
    pub content: GeminiContent1,

    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
}

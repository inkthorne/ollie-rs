use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ===
// STRUCT: GeminiPartCodeExecutable
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiPartCodeExecutable {
    pub language: String,
    pub code: String,
}

// ===
// STRUCT: GeminiPartCode
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiPartCode {
    pub executable_code: GeminiPartCodeExecutable,
}

impl GeminiPartCode {
    pub fn new(language: &str, code: &str) -> Self {
        GeminiPartCode {
            executable_code: GeminiPartCodeExecutable {
                language: language.to_string(),
                code: code.to_string(),
            },
        }
    }
}

// ===
// STRUCT: GeminiPartText
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiPartText {
    pub text: String,
}

// ===
// STRUCT: GeminiPartFunctionCall
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
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
// ENUM: GeminiPart
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiPart {
    Text(GeminiPartText),
    FunctionCall(GeminiPartFunctionCall),
    Code(GeminiPartCode),
}

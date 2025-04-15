use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_json::json;

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
// STRUCT: GeminiFunctionCall
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiFunctionCall {
    #[serde(rename = "functionCall")]
    function_call: GeminiFunctionCallDetails,
}

// ===
// PUBLIC: GeminiFunctionCall
// ===

impl GeminiFunctionCall {
    pub fn name(&self) -> &str {
        &self.function_call.name
    }

    pub fn args(&self) -> &JsonValue {
        &self.function_call.args
    }
}

// ===
// STRUCT: GeminiFunctionCallDetails
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiFunctionCallDetails {
    pub name: String,
    pub args: JsonValue,
}

// ===
// STRUCT: GeminiFunctionResponse
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiFunctionResponse {
    // #[serde(rename = "functionResponse")]
    pub function_response: GeminiFunctionResponseDetails,
}

impl GeminiFunctionResponse {
    pub fn new(name: &str, result: JsonValue) -> Self {
        let response = json!({
            "result": result,
        });

        GeminiFunctionResponse {
            function_response: GeminiFunctionResponseDetails {
                name: name.to_string(),
                response,
            },
        }
    }
}

// ===
// STRUCT: GeminiFunctionResponseDetails
// ===

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiFunctionResponseDetails {
    pub name: String,
    pub response: JsonValue,
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
    Code(GeminiPartCode),
    FunctionCall(GeminiFunctionCall),
    FunctionResponse(GeminiFunctionResponse),
    Text(GeminiPartText),
}

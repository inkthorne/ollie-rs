use serde_json::Value as JsonValue;

// ===
// STRUCT: GeminiResponse
// ===

pub struct GeminiResponse {
    response: JsonValue,
}

// ===
// PUBLIC: GeminiResponse
// ===

impl GeminiResponse {
    pub fn new(response: JsonValue) -> Self {
        GeminiResponse { response }
    }

    pub fn text(&self) -> Option<&str> {
        if let Some(candidates) = self.response.get("candidates") {
            if let Some(candidate) = candidates.get(0) {
                if let Some(content) = candidate.get("content") {
                    if let Some(parts) = content.get("parts") {
                        if let Some(part) = parts.get(0) {
                            if let Some(text) = part.get("text") {
                                if let Some(text_str) = text.as_str() {
                                    return Some(text_str);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

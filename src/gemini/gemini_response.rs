use serde_json::Value as JsonValue;

// ===
// STRUCT: GeminiResponse
// ===

/// Represents a response from the Gemini AI model.
///
/// This struct wraps the JSON response from the Gemini API and provides
/// convenient methods for accessing the response data.
pub struct GeminiResponse {
    response: JsonValue,
}

// ===
// PUBLIC: GeminiResponse
// ===

impl GeminiResponse {
    /// Creates a new GeminiResponse from a JSON value.
    ///
    /// # Arguments
    /// * `response` - The JSON value representing the response from the Gemini API
    ///
    /// # Returns
    /// * A new GeminiResponse instance wrapping the provided JSON
    pub fn new(response: JsonValue) -> Self {
        GeminiResponse { response }
    }

    /// Converts the response to a pretty-printed JSON string.
    ///
    /// # Returns
    /// * A formatted JSON string representation of the response
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.response).unwrap_or_default()
    }

    /// Extracts the primary text content from the response.
    ///
    /// This method navigates through the response structure to find
    /// the first text content from the first candidate.
    ///
    /// # Returns
    /// * Some(&str) containing the text if found, or None if not available
    pub fn text(&self) -> Option<&str> {
        self.response
            .pointer("/candidates/0/content/parts/0/text")?
            .as_str()
    }
}

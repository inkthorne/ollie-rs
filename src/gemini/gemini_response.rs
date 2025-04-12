use serde_json::Value as JsonValue;
use std::fmt;

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

    /// Returns a reference to the underlying JSON value.
    ///
    /// # Returns
    /// * A reference to the JsonValue representing the raw response
    pub fn as_json(&self) -> &JsonValue {
        &self.response
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

    /// Extracts the content object from the first candidate in the response.
    ///
    /// This method navigates through the response structure to find
    /// the content object from the first candidate.
    ///
    /// # Returns
    /// * Some(&JsonValue) containing the content if found, or None if not available
    pub fn content(&self) -> Option<&JsonValue> {
        self.response.pointer("/candidates/0/content")
    }
}

// ===
// TRAIT: GeminiResponse (fmt::Display)
// ===

impl fmt::Display for GeminiResponse {
    /// Formats the GeminiResponse for display using pretty-printed JSON.
    ///
    /// # Arguments
    /// * `f` - The formatter to write the output to
    ///
    /// # Returns
    /// * Result indicating whether the formatting operation succeeded
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_pretty())
    }
}

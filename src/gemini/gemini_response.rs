use crate::{GeminiCandidate, GeminiPart};
use serde::{Deserialize, Serialize};
use std::fmt;

// ===
// STRUCT: GeminiResponse
// ===

/// Represents a response from the Gemini AI API.
///
/// This struct encapsulates the response data received from the Gemini API,
/// providing structured access to the generated content candidates.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    /// The generated candidates from the Gemini model.
    pub candidates: Vec<GeminiCandidate>,
}

// ===
// PUBLIC: GeminiResponse
// ===

impl GeminiResponse {
    /// Converts the response to a pretty-printed JSON string.
    ///
    /// # Returns
    /// * A formatted JSON string representation of the response,
    ///   or an empty string if serialization fails
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_default()
    }

    /// Returns a reference to the content of the first candidate in the response.
    ///
    /// # Returns
    /// * `Some(&GeminiContent1)` if there is at least one candidate in the response
    /// * `None` if there are no candidates
    pub fn content(&self) -> Option<&crate::GeminiContent1> {
        if let Some(candidate) = self.candidates.get(0) {
            return Some(&candidate.content);
        }

        None
    }

    /// Extracts the text from the first part of the first candidate in the response.
    ///
    /// # Returns
    /// * `Some(&str)` containing the text if there is at least one candidate with a text part
    /// * `None` if there are no candidates or the first part isn't text
    pub fn text(&self) -> Option<&str> {
        if let Some(candidate) = self.candidates.get(0) {
            if let GeminiPart::Text(text_part) = &candidate.content.parts[0] {
                return Some(&text_part.text);
            }
        }

        None
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

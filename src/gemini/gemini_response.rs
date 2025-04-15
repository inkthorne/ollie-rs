use crate::{GeminiContent, GeminiFunctionCall, GeminiPart};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

// ===
// STRUCT: GeminiCandidate
// ===

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiCandidate {
    pub index: Option<u32>,
    pub content: GeminiContent,

    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
}

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
    pub candidates: Option<Vec<GeminiCandidate>>,

    /// Information about the error that occurred, if any.
    pub error: Option<JsonValue>,
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
    pub fn content(&self) -> Option<&GeminiContent> {
        if let Some(candidates) = &self.candidates {
            if let Some(candidate) = candidates.get(0) {
                return Some(&candidate.content);
            }
        }

        None
    }

    /// Extracts the text from the first part of the first candidate in the response.
    ///
    /// # Returns
    /// * `Some(&str)` containing the text if there is at least one candidate with a text part
    /// * `None` if there are no candidates or the first part isn't text
    pub fn text(&self) -> Option<&str> {
        if let Some(candidates) = &self.candidates {
            if let Some(candidate) = candidates.get(0) {
                if let GeminiPart::Text(text_part) = &candidate.content.parts[0] {
                    return Some(&text_part.text);
                }
            }
        }

        None
    }

    /// Returns a vector of references to all function call parts in the first candidate's content.
    ///
    /// # Returns
    /// * `Vec<&GeminiPartFunctionCall>` containing all function call parts in the first candidate,
    ///   or an empty vector if there are no candidates or no function call parts.
    pub fn functions(&self) -> Vec<&GeminiFunctionCall> {
        if let Some(candidates) = &self.candidates {
            if let Some(candidate) = candidates.get(0) {
                return candidate
                    .content
                    .parts
                    .iter()
                    .filter_map(|part| {
                        if let GeminiPart::FunctionCall(function_call) = part {
                            Some(function_call)
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }

        Vec::new()
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

// ===
// TRAIT: GeminiResponse (TryFrom<&str>)
// ===

impl TryFrom<&str> for GeminiResponse {
    type Error = serde_json::Error;

    /// Attempts to create a GeminiResponse from a JSON string.
    ///
    /// # Arguments
    /// * `json_str` - A JSON string that can be deserialized into a GeminiResponse
    ///
    /// # Returns
    /// * Result<GeminiResponse, serde_json::Error> - A GeminiResponse instance if parsing succeeds,
    ///   or a serde_json::Error if the parsing fails
    fn try_from(json_str: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json_str)
    }
}

// ===
// TRAIT: GeminiResponse (TryFrom<JsonValue>)
// ===

impl TryFrom<JsonValue> for GeminiResponse {
    type Error = serde_json::Error;

    /// Attempts to create a GeminiResponse from a serde_json::Value.
    ///
    /// # Arguments
    /// * `json_value` - A JSON value that can be converted into a GeminiResponse
    ///
    /// # Returns
    /// * Result<GeminiResponse, serde_json::Error> - A GeminiResponse instance if conversion succeeds,
    ///   or a serde_json::Error if the conversion fails
    fn try_from(json_value: JsonValue) -> Result<Self, Self::Error> {
        serde_json::from_value(json_value)
    }
}

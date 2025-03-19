use crate::message::OllamaMessage;
use bytes::Bytes;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub done_reason: String,
    pub context: Vec<u32>,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
    // pub prompt_eval_count_duration: u64,
    pub eval_count: u32,
    // pub eval_count_duration: u64,
}

//============================================================================
// OllamaResponse
//============================================================================
pub struct OllamaResponse {
    response: serde_json::Value,
}

impl OllamaResponse {
    /// Returns the underlying JSON response as a reference to a serde_json::Value
    ///
    /// ## Returns
    ///
    /// A reference to the underlying JSON value.
    pub fn as_json(&self) -> &serde_json::Value {
        &self.response
    }

    /// Returns a pretty-printed JSON string representation of the response
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of the response.
    /// Returns an empty string if serialization fails.
    pub fn as_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.response).unwrap_or_default()
    }

    /// Extracts the content field from the message in the response
    ///
    /// ## Returns
    ///
    /// The content as a string, or None if the message or content fields are not present.
    pub fn content(&self) -> Option<&str> {
        self.response.get("message")?.get("content")?.as_str()
    }

    /// Checks if the response is done/completed
    ///
    /// ## Returns
    ///
    /// A boolean indicating if the response is done, or None if the done field is not present.
    pub fn done(&self) -> Option<bool> {
        self.response.get("done")?.as_bool()
    }

    /// Gets the error message from the response if present
    ///
    /// ## Returns
    ///
    /// The error message as a string, or None if the error field is not present or not a string.
    pub fn error(&self) -> Option<&str> {
        self.response.get("error")?.as_str()
    }

    /// Extracts the message from the response and converts it to an OllamaMessage
    ///
    /// ## Returns
    ///
    /// An OllamaMessage if the message field is present, or None if the field doesn't exist.
    pub fn message(&self) -> Option<OllamaMessage> {
        self.response.get("message").map(OllamaMessage::from)
    }

    /// Gets the response text from the response
    ///
    /// ## Returns
    ///
    /// The response text as a string, or None if the response field is not present or not a string.
    pub fn response(&self) -> Option<&str> {
        self.response.get("response")?.as_str()
    }
}

/// Implementation to convert a byte slice to an OllamaResponse
///
/// # Errors
///
/// Returns a serde_json::Error if the byte slice cannot be deserialized into a valid JSON value
impl TryFrom<&[u8]> for OllamaResponse {
    type Error = serde_json::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        let response = serde_json::from_slice(slice)?;
        Ok(Self { response })
    }
}

/// Implementation to convert Bytes to an OllamaResponse
///
/// # Errors
///
/// Returns a serde_json::Error if the Bytes cannot be deserialized into a valid JSON value
impl TryFrom<&Bytes> for OllamaResponse {
    type Error = serde_json::Error;

    fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        let response = serde_json::from_slice(bytes)?;
        Ok(Self { response })
    }
}

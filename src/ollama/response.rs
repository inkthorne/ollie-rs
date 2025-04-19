use crate::OllamaMessage;
use bytes::Bytes;

//============================================================================
// OllamaResponse
//============================================================================
/// Wrapper for responses from the Ollama API.
///
/// This struct provides a unified interface for working with responses
/// from different Ollama API endpoints (generate, chat, etc).
pub struct OllamaResponse {
    /// The raw JSON response data
    response: serde_json::Value,
}

impl OllamaResponse {
    /// Creates a new empty OllamaResponse instance
    ///
    /// ## Returns
    ///
    /// A new OllamaResponse with a null JSON value.
    pub fn new() -> Self {
        Self {
            response: serde_json::Value::Null,
        }
    }

    /// Creates a new OllamaResponse instance from a JSON value
    ///
    /// ## Arguments
    ///
    /// * `response` - The JSON value to wrap in the OllamaResponse.
    ///
    /// ## Returns
    ///
    /// A new OllamaResponse containing the provided JSON value.
    pub fn from_json(response: serde_json::Value) -> Self {
        Self { response }
    }

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

    /// Gets the number of tokens generated in the response
    ///
    /// ## Returns
    ///
    /// The eval_count as a u64, or None if the field is not present.
    pub fn eval_count(&self) -> Option<u64> {
        self.response.get("eval_count")?.as_u64()
    }

    /// Gets the duration of the response generation in nanoseconds.
    ///
    /// ## Returns
    ///
    /// The eval_duration as a u64, or None if the field is not present.
    pub fn eval_duration(&self) -> Option<u64> {
        self.response.get("eval_duration")?.as_u64()
    }

    /// Calculates the evaluation rate in tokens per second.
    ///
    /// ## Returns
    ///
    /// The evaluation rate as an f64. Returns 0.0 if the elapsed time is zero.
    pub fn eval_rate(&self) -> f64 {
        let elapsed_time = self.elapsed_time();
        let eval_count = self.eval_count().unwrap_or(0);

        if elapsed_time > 0.0 {
            return eval_count as f64 / elapsed_time;
        }

        0.0
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

    /// Gets the number of tokens in the prompt
    ///
    /// ## Returns
    ///
    /// The prompt_eval_count as a u64, or None if the field is not present.
    pub fn prompt_eval_count(&self) -> Option<u64> {
        self.response.get("prompt_eval_count")?.as_u64()
    }

    /// Gets the response text from the response
    ///
    /// ## Returns
    ///
    /// The response text as a string, or None if the response field is not present or not a string.
    pub fn response(&self) -> Option<&str> {
        self.response.get("response")?.as_str()
    }

    /// Sets the response text in the underlying JSON data
    ///
    /// ## Arguments
    ///
    /// * `text` - The text to set as the response value
    ///
    /// This method modifies the underlying JSON data by setting the "response" field
    /// to the provided string value.
    pub fn set_response(&mut self, text: &str) {
        self.response["response"] = text.into();
    }

    /// Sets the content field in the message object of the response
    ///
    /// ## Arguments
    ///
    /// * `content` - The text to set as the content value
    ///
    /// This method modifies the underlying JSON data by setting the "content" field
    /// within the "message" object of the response. If the message object doesn't
    /// exist, it will be created.
    pub fn set_content(&mut self, content: &str) {
        if !self.response.get("message").is_some() {
            self.response["message"] = serde_json::json!({});
        }
        self.response["message"]["content"] = content.into();
    }

    /// Calculates the elapsed time for the response generation in seconds.
    ///
    /// ## Returns
    ///
    /// The elapsed time in seconds as an f64. Returns 0.0 if the duration is not available.
    pub fn elapsed_time(&self) -> f64 {
        let nanoseconds = self.eval_duration().unwrap_or(0);
        let milliseconds = nanoseconds / 1_000_000;
        let seconds = milliseconds as f64 / 1000.0;
        seconds
    }

    /// Calculates the total number of tokens used in the prompt and response
    ///
    /// ## Returns
    ///
    /// The sum of prompt tokens and response tokens as a u64.
    /// If either count is not available, it defaults to 0.
    pub fn tokens_used(&self) -> u64 {
        let eval_count = self.eval_count().unwrap_or(0);
        let prompt_eval_count = self.prompt_eval_count().unwrap_or(0);

        eval_count + prompt_eval_count
    }

    pub fn print_stats(&self) {
        println!("\n");
        println!("->    eval rate: {:.1} tokens/second", self.eval_rate());
        println!("-> elapsed time: {:.1} seconds", self.elapsed_time());
        println!("->  tokens used: {}", self.tokens_used());
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

//============================================================================
// OllamaRequest
//============================================================================
/// Represents a request to the Ollama API
///
/// This struct is used to build requests for the Ollama API using a fluent interface.
pub struct OllamaRequest {
    value: serde_json::Value,
}

impl OllamaRequest {
    /// Creates a new empty Ollama request
    ///
    /// ## Returns
    ///
    /// A new `OllamaRequest` instance with default values
    pub fn new() -> Self {
        Self {
            value: serde_json::Value::default(),
        }
    }

    /// Sets the model to use for the request
    ///
    /// ## Arguments
    ///
    /// * `model` - The name of the model to use (e.g., "gemma3:4b", "llama3")
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn model(&mut self, model: String) -> &mut Self {
        self.value["model"] = serde_json::Value::String(model);
        self
    }

    /// Sets the prompt text for the request
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The prompt text to send to the model
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn prompt(&mut self, prompt: String) -> &mut Self {
        self.value["prompt"] = serde_json::Value::String(prompt);
        self
    }

    /// Sets whether the response should be streamed
    ///
    /// ## Arguments
    ///
    /// * `prompt` - Boolean indicating if the response should be streamed
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn stream(&mut self, prompt: bool) -> &mut Self {
        self.value["stream"] = serde_json::Value::Bool(prompt);
        self
    }

    /// Sets the requested output format
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The format to request (e.g., "json")
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn format(&mut self, prompt: String) -> &mut Self {
        self.value["format"] = serde_json::Value::String(prompt);
        self
    }

    /// Returns the underlying JSON value of the request
    ///
    /// ## Returns
    ///
    /// A reference to the internal JSON value
    pub fn as_json(&self) -> &serde_json::Value {
        &self.value
    }
}

//============================================================================
// OllamaRequest
//============================================================================
/// Represents a request to the Ollama API
///
/// This struct is used to build requests for the Ollama API using a fluent interface.
pub struct OllamaRequest {
    request: serde_json::Value,
}

impl OllamaRequest {
    /// Creates a new empty Ollama request
    ///
    /// ## Returns
    ///
    /// A new `OllamaRequest` instance with default values
    pub fn new() -> Self {
        Self {
            request: serde_json::Value::default(),
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
    pub fn model(&mut self, model: &str) -> &mut Self {
        self.request["model"] = model.into();
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
    pub fn prompt(&mut self, prompt: &str) -> &mut Self {
        self.request["prompt"] = prompt.into();
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
    pub fn stream(&mut self, stream: bool) -> &mut Self {
        self.request["stream"] = stream.into();
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
    pub fn format(&mut self, format: &str) -> &mut Self {
        self.request["format"] = format.into();
        self
    }

    /// Returns the underlying JSON value of the request
    ///
    /// ## Returns
    ///
    /// A reference to the internal JSON value
    pub fn as_json(&self) -> &serde_json::Value {
        &self.request
    }
}

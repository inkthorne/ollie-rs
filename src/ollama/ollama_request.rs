use crate::OllamaResponse2;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

// ===
// STRUCT: OllamaRequest2
// ===

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaRequest2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<JsonValue>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<JsonValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

impl OllamaRequest2 {
    /// Creates a new, empty `OllamaRequest2`.
    ///
    /// All fields are initialized to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `OllamaRequest2`.
    pub fn new() -> Self {
        Self {
            model: None,
            messages: None,
            options: None,
            prompt: None,
            stream: None,
        }
    }

    /// Creates an `OllamaRequest2` instance from a JSON value.
    ///
    /// # Arguments
    ///
    /// * `json` - A `serde_json::Value` representing the Ollama request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized `OllamaRequest2` on success,
    /// or a `serde_json::Error` if deserialization fails.
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if the provided JSON value cannot be
    /// deserialized into an `OllamaRequest2`.
    pub fn from_json(json: JsonValue) -> Result<Self, serde_json::Error> {
        let request = serde_json::from_value(json)?;
        Ok(request)
    }

    /// Converts the `OllamaRequest2` instance into a JSON value.
    ///
    /// This method consumes the `OllamaRequest2` instance.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the serialized `OllamaRequest2`.
    /// Panics if serialization fails (which should generally not happen for this struct).
    pub fn to_json(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }

    /// Returns a reference to the model name, if set.
    ///
    /// # Returns
    ///
    /// An `Option<&String>` containing the model name.
    pub fn model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the model name for the request.
    ///
    /// # Arguments
    ///
    /// * `model` - A string slice representing the model name.
    ///
    /// # Returns
    ///
    /// The modified `OllamaRequest2` instance.
    pub fn set_model(mut self, model: &str) -> Self {
        self.model = Some(model.to_string());
        self
    }

    /// Returns a reference to the vector of messages, if set.
    ///
    /// # Returns
    ///
    /// An `Option<&Vec<JsonValue>>` containing the messages.
    pub fn messages(&self) -> Option<&Vec<JsonValue>> {
        self.messages.as_ref()
    }

    /// Sets the messages for the request.
    ///
    /// # Arguments
    ///
    /// * `messages` - A vector of `serde_json::Value` representing the messages.
    ///
    /// # Returns
    ///
    /// The modified `OllamaRequest2` instance.
    pub fn set_messages(mut self, messages: Vec<JsonValue>) -> Self {
        self.messages = Some(messages);
        self
    }

    /// Adds a single message to the request's message list.
    ///
    /// If the message list does not exist, it will be created.
    ///
    /// # Arguments
    ///
    /// * `message` - A `serde_json::Value` representing the message to add.
    ///
    /// # Returns
    ///
    /// The modified `OllamaRequest2` instance.
    pub fn add_message(mut self, message: JsonValue) -> Self {
        match &mut self.messages {
            Some(messages) => messages.push(message),
            None => self.messages = Some(vec![message]),
        }
        self
    }

    /// Returns a reference to the options JSON value, if set.
    ///
    /// # Returns
    ///
    /// An `Option<&JsonValue>` containing the options.
    pub fn options(&self) -> Option<&JsonValue> {
        self.options.as_ref()
    }

    /// Sets the options for the request.
    ///
    /// # Arguments
    ///
    /// * `options` - A `serde_json::Value` representing the options.
    ///
    /// # Returns
    ///
    /// The modified `OllamaRequest2` instance.
    pub fn set_options(mut self, options: JsonValue) -> Self {
        self.options = Some(options);
        self
    }

    /// Returns a reference to the prompt string, if set.
    ///
    /// # Returns
    ///
    /// An `Option<&String>` containing the prompt.
    pub fn prompt(&self) -> Option<&String> {
        self.prompt.as_ref()
    }

    /// Sets the prompt string for the request.
    ///
    /// # Arguments
    ///
    /// * `prompt` - A string slice representing the prompt.
    ///
    /// # Returns
    ///
    /// The modified `OllamaRequest2` instance.
    pub fn set_prompt(mut self, prompt: &str) -> Self {
        self.prompt = Some(prompt.to_string());
        self
    }

    /// Adds the message content from an Ollama response JSON to the request's messages.
    ///
    /// This method looks for a "message" field within the provided `response` JSON.
    /// If found, its value is cloned and added to the `messages` list using `add_message`.
    /// If the "message" field is not present, the request remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `response` - A `serde_json::Value` representing the Ollama response.
    ///
    /// # Returns
    ///
    /// The potentially modified `OllamaRequest2` instance.
    pub fn add_response(self, response: &OllamaResponse2) -> Self {
        if let Some(message) = response.message() {
            let message_json = message.clone().to_json();
            return self.add_message(message_json);
        }

        self
    }

    /// Returns the stream setting, if set.
    ///
    /// # Returns
    ///
    /// An `Option<bool>` indicating whether streaming is enabled.
    pub fn stream(&self) -> Option<bool> {
        self.stream
    }

    /// Sets the stream setting for the request.
    ///
    /// # Arguments
    ///
    /// * `stream` - A boolean indicating whether to enable streaming.
    ///
    /// # Returns
    ///
    /// The modified `OllamaRequest2` instance.
    pub fn set_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

// ===
// TRAIT: Display for OllamaRequest2
// ===

impl fmt::Display for OllamaRequest2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_json = serde_json::to_value(&self).unwrap();
        let pretty_string = serde_json::to_string_pretty(&request_json).unwrap();
        write!(f, "{pretty_string}")
    }
}

// ===
// TESTS: OllamaRequest2
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let req = OllamaRequest2::new();
        assert!(req.model.is_none());
        assert!(req.messages.is_none());
        assert!(req.options.is_none());
        assert!(req.stream.is_none());
    }

    #[test]
    fn test_setters_getters() {
        let messages = vec![json!({"role": "user", "content": "Hello"})];
        let options = json!({"temperature": 0.8});

        let req = OllamaRequest2::new()
            .set_model("llama2")
            .set_messages(messages.clone())
            .set_options(options.clone())
            .set_stream(true);

        assert_eq!(req.model(), Some(&"llama2".to_string()));
        assert_eq!(req.messages(), Some(&messages));
        assert_eq!(req.options(), Some(&options));
        assert_eq!(req.stream(), Some(true));
    }

    #[test]
    fn test_add_message() {
        let msg1 = json!({"role": "user", "content": "First message"});
        let msg2 = json!({"role": "assistant", "content": "Second message"});

        let req = OllamaRequest2::new().add_message(msg1.clone());
        assert_eq!(req.messages(), Some(&vec![msg1.clone()]));

        let req = req.add_message(msg2.clone());
        assert_eq!(req.messages(), Some(&vec![msg1, msg2]));
    }

    #[test]
    fn test_to_json_full() {
        let messages = vec![json!({"role": "user", "content": "Test"})];
        let options = json!({"seed": 123});
        let req = OllamaRequest2::new()
            .set_model("test-model")
            .set_messages(messages.clone())
            .set_options(options.clone())
            .set_stream(false);

        let expected_json = json!({
            "model": "test-model",
            "messages": messages,
            "options": options,
            "stream": false
        });

        assert_eq!(req.to_json(), expected_json);
    }

    #[test]
    fn test_to_json_minimal() {
        let req = OllamaRequest2::new().set_model("minimal-model");

        let expected_json = json!({
            "model": "minimal-model"
        });
        // Clone req before calling to_json which consumes self
        let req_clone = req.clone();
        assert_eq!(req_clone.to_json(), expected_json);
        assert!(req.messages.is_none()); // Verify other fields remain None
        assert!(req.options.is_none());
        assert!(req.stream.is_none());
    }

    #[test]
    fn test_from_json_full() {
        let json_data = json!({
            "model": "test-model",
            "messages": [{"role": "user", "content": "Test"}],
            "options": {"seed": 456},
            "stream": true
        });

        let req = OllamaRequest2::from_json(json_data.clone()).unwrap();

        assert_eq!(req.model(), Some(&"test-model".to_string()));
        assert!(req.messages().is_some());
        assert_eq!(req.messages().unwrap().len(), 1);
        assert_eq!(
            req.messages().unwrap()[0],
            json!({"role": "user", "content": "Test"})
        );
        assert_eq!(req.options(), Some(&json!({"seed": 456})));
        assert_eq!(req.stream(), Some(true));
    }

    #[test]
    fn test_from_json_partial() {
        let json_data = json!({
            "model": "partial-model",
            "stream": false
        });

        let req = OllamaRequest2::from_json(json_data).unwrap();

        assert_eq!(req.model(), Some(&"partial-model".to_string()));
        assert!(req.messages().is_none());
        assert!(req.options().is_none());
        assert_eq!(req.stream(), Some(false));
    }

    #[test]
    fn test_from_json_invalid() {
        // Missing required field 'model' if it were required, but it's optional
        // Let's test invalid type instead
        let json_data = json!({
            "model": 123 // Invalid type for model
        });
        let result = OllamaRequest2::from_json(json_data);
        assert!(result.is_err());

        let json_data_invalid_message = json!({
            "model": "test",
            "messages": "not an array" // Invalid type for messages
        });
        let result_invalid_message = OllamaRequest2::from_json(json_data_invalid_message);
        assert!(result_invalid_message.is_err());
    }

    #[test]
    fn test_prompt_setter_getter() {
        let req = OllamaRequest2::new().set_prompt("This is a test prompt.");
        assert_eq!(req.prompt(), Some(&"This is a test prompt.".to_string()));

        let req_none = OllamaRequest2::new();
        assert!(req_none.prompt().is_none());
    }

    #[test]
    fn test_add_response() {
        let response_with_message_json = json!({
            "model": "llama2",
            "created_at": "2023-08-04T08:52:19.385406455Z",
            "message": {
                "role": "assistant",
                "content": "Response message content"
            },
            "done": true
        });
        let response_with_message = OllamaResponse2::from_json(response_with_message_json).unwrap();
        let response_without_message_json = json!({
            "model": "llama2",
            "created_at": "2023-08-04T08:52:19.385406455Z",
            "done": true
        });
        let response_without_message =
            OllamaResponse2::from_json(response_without_message_json).unwrap();

        let expected_message = json!({
            "role": "assistant",
            "content": "Response message content"
        });

        // Test adding response when messages is None
        let req1 = OllamaRequest2::new();
        let req1_updated = req1.add_response(&response_with_message);
        assert!(req1_updated.messages().is_some());
        assert_eq!(req1_updated.messages().unwrap().len(), 1);
        assert_eq!(req1_updated.messages().unwrap()[0], expected_message);

        // Test adding response when messages already exists
        let initial_message = json!({"role": "user", "content": "Initial prompt"});
        let req2 = OllamaRequest2::new().add_message(initial_message.clone());
        let req2_updated = req2.add_response(&response_with_message);
        assert!(req2_updated.messages().is_some());
        assert_eq!(req2_updated.messages().unwrap().len(), 2);
        assert_eq!(req2_updated.messages().unwrap()[0], initial_message);
        assert_eq!(req2_updated.messages().unwrap()[1], expected_message);

        // Test adding response without a message field
        let req3 = OllamaRequest2::new();
        let req3_updated = req3.add_response(&response_without_message);
        assert!(req3_updated.messages().is_none()); // Should remain None

        let req4 = OllamaRequest2::new().add_message(initial_message.clone());
        let req4_updated = req4.add_response(&response_without_message);
        assert!(req4_updated.messages().is_some());
        assert_eq!(req4_updated.messages().unwrap().len(), 1); // Should remain unchanged
        assert_eq!(req4_updated.messages().unwrap()[0], initial_message);
    }

    #[test]
    fn test_to_json_with_prompt() {
        let req = OllamaRequest2::new()
            .set_model("test-model")
            .set_prompt("Test prompt");

        let expected_json = json!({
            "model": "test-model",
            "prompt": "Test prompt"
        });
        // Clone req before calling to_json which consumes self
        let req_clone = req.clone();
        assert_eq!(req_clone.to_json(), expected_json);
    }

    #[test]
    fn test_from_json_with_prompt() {
        let json_data = json!({
            "model": "test-model",
            "prompt": "Another test prompt"
        });

        let req = OllamaRequest2::from_json(json_data).unwrap();

        assert_eq!(req.model(), Some(&"test-model".to_string()));
        assert_eq!(req.prompt(), Some(&"Another test prompt".to_string()));
        assert!(req.messages().is_none());
        assert!(req.options().is_none());
        assert!(req.stream().is_none());
    }
}

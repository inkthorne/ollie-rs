use crate::OllamaResponse2;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

// ===
// STRUCT: OllamaRequest
// ===

#[derive(Serialize, Deserialize)]
pub struct OllamaRequest {
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

impl OllamaRequest {
    /// Creates a new, empty `OllamaRequest`.
    ///
    /// All fields are initialized to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `OllamaRequest`.
    pub fn new() -> Self {
        Self {
            model: None,
            messages: None,
            options: None,
            prompt: None,
            stream: None,
        }
    }

    /// Creates an `OllamaRequest` instance from a JSON value.
    ///
    /// # Arguments
    ///
    /// * `json` - A `serde_json::Value` representing the Ollama request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized `OllamaRequest` on success,
    /// or a `serde_json::Error` if deserialization fails.
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if the provided JSON value cannot be
    /// deserialized into an `OllamaRequest`.
    pub fn from_json(json: JsonValue) -> Result<Self, serde_json::Error> {
        let request = serde_json::from_value(json)?;
        Ok(request)
    }

    /// Converts the `OllamaRequest` instance into a JSON value.
    ///
    /// This method consumes the `OllamaRequest` instance.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the serialized `OllamaRequest`.
    /// Panics if serialization fails (which should generally not happen for this struct).
    pub fn to_json(&self) -> JsonValue {
        serde_json::to_value(&self).unwrap()
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
    /// The modified `OllamaRequest` instance.
    pub fn set_model(&mut self, model: &str) -> &mut Self {
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
    /// The modified `OllamaRequest` instance.
    pub fn set_messages(&mut self, messages: &Vec<JsonValue>) -> &mut Self {
        self.messages = Some(messages.clone());
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
    /// The modified `OllamaRequest` instance.
    pub fn add_message(&mut self, message: JsonValue) -> &mut Self {
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
    /// The modified `OllamaRequest` instance.
    pub fn set_options(&mut self, options: &JsonValue) -> &mut Self {
        self.options = Some(options.clone());
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
    /// The modified `OllamaRequest` instance.
    pub fn set_prompt(&mut self, prompt: &str) -> &mut Self {
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
    /// The potentially modified `OllamaRequest` instance.
    pub fn add_response(&mut self, response: &OllamaResponse2) -> &mut Self {
        if let Some(message) = response.message() {
            let message_json = message.to_json();
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
    /// The modified `OllamaRequest` instance.
    pub fn set_stream(&mut self, stream: bool) -> &mut Self {
        self.stream = Some(stream);
        self
    }
}

// ===
// TRAIT: Display for OllamaRequest
// ===

impl fmt::Display for OllamaRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_json = serde_json::to_value(&self).unwrap();
        let pretty_string = serde_json::to_string_pretty(&request_json).unwrap();
        write!(f, "{pretty_string}")
    }
}

// ===
// TESTS: OllamaRequest
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let req = OllamaRequest::new();
        assert!(req.model.is_none());
        assert!(req.messages.is_none());
        assert!(req.options.is_none());
        assert!(req.stream.is_none());
    }

    #[test]
    fn test_setters_getters() {
        let messages = vec![json!({"role": "user", "content": "Hello"})];
        let options = json!({"temperature": 0.8});

        let mut req = OllamaRequest::new();
        req.set_model("llama2")
            .set_messages(&messages)
            .set_options(&options)
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

        let mut req = OllamaRequest::new();
        req.add_message(msg1.clone());
        assert_eq!(req.messages(), Some(&vec![msg1.clone()]));

        let req = req.add_message(msg2.clone());
        assert_eq!(req.messages(), Some(&vec![msg1, msg2]));
    }

    #[test]
    fn test_to_json_full() {
        let messages = vec![json!({"role": "user", "content": "Test"})];
        let options = json!({"seed": 123});
        let mut req = OllamaRequest::new();
        req.set_model("test-model")
            .set_messages(&messages)
            .set_options(&options)
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
        let mut req = OllamaRequest::new();
        req.set_model("minimal-model");

        let expected_json = json!({
            "model": "minimal-model"
        });

        assert_eq!(req.to_json(), expected_json);
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

        let req = OllamaRequest::from_json(json_data.clone()).unwrap();

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

        let req = OllamaRequest::from_json(json_data).unwrap();

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
        let result = OllamaRequest::from_json(json_data);
        assert!(result.is_err());

        let json_data_invalid_message = json!({
            "model": "test",
            "messages": "not an array" // Invalid type for messages
        });
        let result_invalid_message = OllamaRequest::from_json(json_data_invalid_message);
        assert!(result_invalid_message.is_err());
    }

    #[test]
    fn test_prompt_setter_getter() {
        let mut req = OllamaRequest::new();
        req.set_prompt("This is a test prompt.");
        assert_eq!(req.prompt(), Some(&"This is a test prompt.".to_string()));

        let req_none = OllamaRequest::new();
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
        let mut req1 = OllamaRequest::new();
        req1.add_response(&response_with_message);
        assert!(req1.messages().is_some());
        assert_eq!(req1.messages().unwrap().len(), 1);
        assert_eq!(req1.messages().unwrap()[0], expected_message);

        // Test adding response when messages already exists
        let initial_message = json!({"role": "user", "content": "Initial prompt"});
        let mut req2 = OllamaRequest::new();
        req2.add_message(initial_message.clone());
        req2.add_response(&response_with_message);
        assert!(req2.messages().is_some());
        assert_eq!(req2.messages().unwrap().len(), 2);
        assert_eq!(req2.messages().unwrap()[0], initial_message);
        assert_eq!(req2.messages().unwrap()[1], expected_message);

        // Test adding response without a message field
        let mut req3 = OllamaRequest::new();
        req3.add_response(&response_without_message);
        assert!(req3.messages().is_none()); // Should remain None

        let mut req4 = OllamaRequest::new();
        req4.add_message(initial_message.clone());
        req4.add_response(&response_without_message);
        assert!(req4.messages().is_some());
        assert_eq!(req4.messages().unwrap().len(), 1); // Should remain unchanged
        assert_eq!(req4.messages().unwrap()[0], initial_message);
    }

    #[test]
    fn test_to_json_with_prompt() {
        let mut req = OllamaRequest::new();
        req.set_model("test-model").set_prompt("Test prompt");

        let expected_json = json!({
            "model": "test-model",
            "prompt": "Test prompt"
        });

        assert_eq!(req.to_json(), expected_json);
    }

    #[test]
    fn test_from_json_with_prompt() {
        let json_data = json!({
            "model": "test-model",
            "prompt": "Another test prompt"
        });

        let req = OllamaRequest::from_json(json_data).unwrap();

        assert_eq!(req.model(), Some(&"test-model".to_string()));
        assert_eq!(req.prompt(), Some(&"Another test prompt".to_string()));
        assert!(req.messages().is_none());
        assert!(req.options().is_none());
        assert!(req.stream().is_none());
    }
}

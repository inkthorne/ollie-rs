//============================================================================
// OllamaMessages
//============================================================================
/// A struct representing a single message for Ollama API communication.
///
/// This struct wraps a JSON value that contains role and content fields,
/// which are required for communicating with Ollama API endpoints.
/// It provides a builder pattern interface for constructing messages.
pub struct OllamaMessage {
    message: serde_json::Value,
}

impl OllamaMessage {
    /// Creates a new empty OllamaMessage instance.
    ///
    /// ## Returns
    ///
    /// A new OllamaMessage with default JSON value.
    pub fn new() -> Self {
        Self {
            message: serde_json::Value::default(),
        }
    }

    pub fn as_json(&self) -> &serde_json::Value {
        &self.message
    }

    /// Sets the role field of the message.
    ///
    /// ## Arguments
    ///
    /// * `role` - The role string to set (e.g., "user", "assistant")
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn role(&mut self, role: &str) -> &mut Self {
        self.message["role"] = role.into();
        self
    }

    /// Sets the content field of the message.
    ///
    /// ## Arguments
    ///
    /// * `content` - The content string to set
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn content(&mut self, content: &str) -> &mut Self {
        self.message["content"] = content.into();
        self
    }

    /// Converts the message to a pretty-printed JSON string.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of the message.
    /// Returns an empty string if serialization fails.
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.message).unwrap_or_default()
    }
}

//============================================================================
// OllamaMessages
//============================================================================
/// A collection of messages for Ollama API communication.
///
/// This struct maintains an array of messages that can be sent to Ollama API endpoints.
/// It handles the proper formatting of message collections and provides methods
/// for adding messages to the collection.
pub struct OllamaMessages {
    messages: serde_json::Value,
}

impl OllamaMessages {
    /// Creates a new empty OllamaMessages collection.
    ///
    /// ## Returns
    ///
    /// A new OllamaMessages with an empty array.
    pub fn new() -> Self {
        Self {
            messages: serde_json::Value::Array(vec![]),
        }
    }

    pub fn as_json(&self) -> &serde_json::Value {
        &self.messages
    }

    /// Adds a message to the collection.
    ///
    /// ## Arguments
    ///
    /// * `message` - The OllamaMessage to add to the collection
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn push_message(&mut self, message: OllamaMessage) -> &mut Self {
        self.messages.as_array_mut().unwrap().push(message.message);
        self
    }

    /// Converts the message collection to a pretty-printed JSON string.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of all messages.
    /// Returns an empty string if serialization fails.
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.messages).unwrap_or_default()
    }
}

//============================================================================
// TESTS
//============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the OllamaMessage struct functionality.
    ///
    /// This test verifies that:
    /// - A new message can be created
    /// - Role and content can be set using the builder pattern
    /// - Fields can be properly accessed and modified
    /// - The message can be converted to a pretty-printed JSON string
    #[test]
    fn test_json_message() {
        // Create a new message
        let mut message = OllamaMessage::new();

        // Set role and content using builder pattern
        message.role("user").content("Hello, world!");

        // Verify the role field
        assert_eq!(message.message["role"], "user");

        // Verify the content field
        assert_eq!(message.message["content"], "Hello, world!");

        // Verify we can change values
        message.role("assistant").content("I'm an AI assistant");
        assert_eq!(message.message["role"], "assistant");
        assert_eq!(message.message["content"], "I'm an AI assistant");

        // Verify we can convert to a pretty string
        println!("---\nmessage: {}", message.to_string_pretty());
    }

    /// Tests the OllamaMessages collection functionality.
    ///
    /// This test verifies that:
    /// - A new messages collection can be created
    /// - Individual OllamaMessage instances can be added to the collection
    /// - The builder pattern works for chaining operations
    /// - The collection can be converted to a properly formatted JSON string
    /// - The resulting JSON contains all expected message content
    #[test]
    fn test_json_messages() {
        // Create a new messages collection
        let mut messages = OllamaMessages::new();

        // Create the first message
        let mut user_message = OllamaMessage::new();
        user_message
            .role("user")
            .content("What is the capital of France?");

        // Create the second message
        let mut assistant_message = OllamaMessage::new();
        assistant_message
            .role("assistant")
            .content("The capital of France is Paris.");

        // Add both messages to the collection using builder pattern
        messages
            .push_message(user_message)
            .push_message(assistant_message);

        // Get the JSON string representation
        let json_str = messages.to_string_pretty();
        println!("---\nmessages: {}", json_str);

        // Verify the JSON contains our messages
        assert!(json_str.contains("What is the capital of France?"));
        assert!(json_str.contains("The capital of France is Paris."));
        assert!(json_str.contains("user"));
        assert!(json_str.contains("assistant"));
    }
}

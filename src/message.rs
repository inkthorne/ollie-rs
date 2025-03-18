//============================================================================
// OllamaMessage
//============================================================================
/// A struct representing a single message for Ollama API communication.
///
/// This struct wraps a JSON value that contains role and content fields,
/// which are required for communicating with Ollama API endpoints.
/// It provides a builder pattern interface for constructing messages.
///
/// ## Example JSON structure
/// ```json
/// {
///     "role": "user",
///     "content": "why is the sky blue?"
/// }
/// ```
pub struct OllamaMessage {
    object: serde_json::Value,
}

impl OllamaMessage {
    /// Creates a new empty OllamaMessage instance.
    ///
    /// ## Returns
    ///
    /// A new OllamaMessage with default JSON value.
    pub fn new() -> Self {
        Self {
            object: serde_json::Value::default(),
        }
    }

    /// Gets the underlying JSON representation of the message.
    ///
    /// ## Returns
    ///
    /// A reference to the underlying JSON value.
    pub fn as_json(&self) -> &serde_json::Value {
        &self.object
    }

    /// Converts the message to a pretty-printed JSON string.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of the message.
    /// Returns an empty string if serialization fails.
    pub fn as_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.object).unwrap_or_default()
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
    pub fn set_content(&mut self, content: &str) -> &mut Self {
        self.object["content"] = content.into();
        self
    }

    /// Gets the content field of the message.
    ///
    /// ## Returns
    ///
    /// The content as a string, or None if the field doesn't exist.
    pub fn content(&self) -> Option<&str> {
        self.object.get("content")?.as_str()
    }

    /// Sets the name field of the message.
    ///
    /// ## Arguments
    ///
    /// * `name` - The name string to set (typically used to identify the source of a message)
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.object["name"] = name.into();
        self
    }

    /// Gets the name field of the message.
    ///
    /// ## Returns
    ///
    /// The name as a string, or None if the field doesn't exist.
    pub fn name(&self) -> Option<&str> {
        self.object.get("name")?.as_str()
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
    pub fn set_role(&mut self, role: &str) -> &mut Self {
        self.object["role"] = role.into();
        self
    }

    /// Gets the role field of the message.
    ///
    /// ## Returns
    ///
    /// The role as a string, or None if the field doesn't exist.
    pub fn role(&self) -> Option<&str> {
        self.object.get("role")?.as_str()
    }
}

/// Implementation of the From trait for converting a reference to serde_json::Value into an OllamaMessage
impl From<&serde_json::Value> for OllamaMessage {
    /// Creates an OllamaMessage from a reference to serde_json::Value.
    ///
    /// ## Arguments
    ///
    /// * `value` - The JSON value reference to convert into an OllamaMessage
    ///
    /// ## Returns
    ///
    /// A new OllamaMessage containing a clone of the provided JSON value.
    fn from(value: &serde_json::Value) -> Self {
        Self {
            object: value.clone(),
        }
    }
}

// Keep the original implementation for backward compatibility
impl From<serde_json::Value> for OllamaMessage {
    /// Creates an OllamaMessage from a serde_json::Value.
    ///
    /// ## Arguments
    ///
    /// * `value` - The JSON value to convert into an OllamaMessage
    ///
    /// ## Returns
    ///
    /// A new OllamaMessage containing the provided JSON value.
    fn from(value: serde_json::Value) -> Self {
        Self { object: value }
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
///
/// ## Example JSON structure
/// ```json
/// [
///   {
///     "role": "user",
///     "content": "why is the sky blue?"
///   },
///   {
///     "role": "assistant",
///     "content": "due to rayleigh scattering."
///   },
///   {
///     "role": "user",
///     "content": "how is that different than mie scattering?"
///   }
/// ]
/// ```
pub struct OllamaMessages {
    array: serde_json::Value,
}

impl OllamaMessages {
    /// Creates a new empty OllamaMessages collection.
    ///
    /// ## Returns
    ///
    /// A new OllamaMessages with an empty array.
    pub fn new() -> Self {
        Self {
            array: serde_json::Value::Array(vec![]),
        }
    }

    /// Gets the underlying JSON representation of the message collection.
    ///
    /// ## Returns
    ///
    /// A reference to the underlying JSON value containing the message array.
    pub fn as_json(&self) -> &serde_json::Value {
        &self.array
    }

    /// Converts the message collection to a pretty-printed JSON string.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of all messages.
    /// Returns an empty string if serialization fails.
    pub fn as_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.array).unwrap_or_default()
    }

    /// Returns the number of messages in the collection.
    ///
    /// ## Returns
    ///
    /// An integer count of the messages in the array.
    pub fn len(&self) -> usize {
        match self.array.as_array() {
            Some(arr) => arr.len(),
            None => 0,
        }
    }

    /// Retrieves a message at the specified index.
    ///
    /// ## Arguments
    ///
    /// * `index` - The zero-based index of the message to retrieve
    ///
    /// ## Returns
    ///
    /// An Option containing the OllamaMessage if the index is valid, or None if out of bounds.
    pub fn message(&self, index: usize) -> Option<OllamaMessage> {
        match self.array.as_array() {
            Some(arr) => arr.get(index).map(OllamaMessage::from),
            None => None,
        }
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
        self.array.as_array_mut().unwrap().push(message.object);
        self
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
        message.set_role("user").set_content("Hello, world!");

        // Verify the role field
        assert_eq!(message.object["role"], "user");

        // Verify the content field
        assert_eq!(message.object["content"], "Hello, world!");

        // Verify we can change values
        message
            .set_role("assistant")
            .set_content("I'm an AI assistant");
        assert_eq!(message.object["role"], "assistant");
        assert_eq!(message.object["content"], "I'm an AI assistant");

        // Verify we can convert to a pretty string
        println!("---\nmessage: {}", message.as_string_pretty());
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
            .set_role("user")
            .set_content("What is the capital of France?");

        // Create the second message
        let mut assistant_message = OllamaMessage::new();
        assistant_message
            .set_role("assistant")
            .set_content("The capital of France is Paris.");

        // Add both messages to the collection using builder pattern
        messages
            .push_message(user_message)
            .push_message(assistant_message);

        // Get the JSON string representation
        let json_str = messages.as_string_pretty();
        println!("---\nmessages: {}", json_str);

        // Verify the JSON contains our messages
        assert!(json_str.contains("What is the capital of France?"));
        assert!(json_str.contains("The capital of France is Paris."));
        assert!(json_str.contains("user"));
        assert!(json_str.contains("assistant"));
    }

    /// Tests the From<serde_json::Value> implementation for OllamaMessage.
    ///
    /// This test verifies that:
    /// - An OllamaMessage can be created directly from a serde_json::Value
    /// - The value fields are correctly accessible after conversion
    #[test]
    fn test_from_json_value() {
        // Create a JSON object with role and content
        let json = serde_json::json!({
            "role": "user",
            "content": "Test content from JSON"
        });

        // Convert to OllamaMessage using From trait
        let message = OllamaMessage::from(json);

        // Verify the fields
        assert_eq!(message.role(), Some("user"));
        assert_eq!(message.content(), Some("Test content from JSON"));
    }

    /// Tests the From<&serde_json::Value> implementation for OllamaMessage.
    ///
    /// This test verifies that:
    /// - An OllamaMessage can be created from a reference to serde_json::Value
    /// - The value fields are correctly accessible after conversion
    /// - The original value is not consumed and can still be accessed
    #[test]
    fn test_from_json_value_reference() {
        // Create a JSON object with role and content
        let json = serde_json::json!({
            "role": "assistant",
            "content": "Test content from JSON reference"
        });

        // Convert to OllamaMessage using From trait with reference
        let message = OllamaMessage::from(&json);

        // Verify the fields
        assert_eq!(message.role(), Some("assistant"));
        assert_eq!(message.content(), Some("Test content from JSON reference"));

        // Verify the original value still exists and is unchanged
        assert_eq!(json["role"], "assistant");
        assert_eq!(json["content"], "Test content from JSON reference");
    }

    /// Tests the message accessor method of OllamaMessages.
    ///
    /// This test verifies that:
    /// - Messages can be accessed by index
    /// - Out of bounds indices return None
    /// - Accessed messages contain the expected content
    #[test]
    fn test_message_accessor() {
        // Create a collection with two messages
        let mut messages = OllamaMessages::new();

        let mut first_message = OllamaMessage::new();
        first_message.set_role("user").set_content("First message");

        let mut second_message = OllamaMessage::new();
        second_message
            .set_role("assistant")
            .set_content("Second message");

        messages
            .push_message(first_message)
            .push_message(second_message);

        // Test accessing by valid indices
        let message0 = messages.message(0);
        assert!(message0.is_some());
        assert_eq!(message0.unwrap().role(), Some("user"));

        let message1 = messages.message(1);
        assert!(message1.is_some());
        assert_eq!(message1.unwrap().content(), Some("Second message"));

        // Test accessing by invalid index
        let message_out_of_bounds = messages.message(2);
        assert!(message_out_of_bounds.is_none());
    }
}

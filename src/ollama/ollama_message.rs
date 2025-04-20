use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ===
// STRUCT: OllamaMessage2
// ===

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaMessage2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

impl OllamaMessage2 {
    /// Creates a new, empty `OllamaMessage2`.
    ///
    /// Both `role` and `content` fields are initialized to `None`.
    pub fn new() -> Self {
        OllamaMessage2 {
            role: None,
            content: None,
        }
    }

    /// Deserializes an `OllamaMessage2` from a `serde_json::Value`.
    ///
    /// # Arguments
    ///
    /// * `json` - A `serde_json::Value` representing the message.
    ///
    /// # Errors
    ///
    /// Returns `serde_json::Error` if deserialization fails.
    pub fn from_json(json: JsonValue) -> Result<Self, serde_json::Error> {
        let message = serde_json::from_value(json)?;
        Ok(message)
    }

    /// Serializes the `OllamaMessage2` into a `serde_json::Value`.
    ///
    /// Fields that are `None` will be skipped during serialization.
    ///
    /// # Panics
    ///
    /// Panics if serialization fails, which should generally not happen for this struct.
    pub fn to_json(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }

    /// Returns the role of the message.
    ///
    /// Returns `None` if the role is not set.
    pub fn role(&self) -> Option<&str> {
        self.role.as_deref()
    }

    /// Sets the role of the message.
    ///
    /// # Arguments
    ///
    /// * `role` - The role to set (e.g., "user", "assistant", "system").
    ///
    /// Returns the modified `OllamaMessage2` instance.
    pub fn set_role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    /// Returns the content of the message.
    ///
    /// Returns `None` if the content is not set.
    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    /// Sets the content of the message.
    ///
    /// # Arguments
    ///
    /// * `content` - The message content.
    ///
    /// Returns the modified `OllamaMessage2` instance.
    pub fn set_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }
}

// ===
// TESTS: OllamaMessage2
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let msg = OllamaMessage2::new();
        // Check that getters return None for None fields
        assert_eq!(msg.role(), None); // Fixed: Compare with None
        // Check that content() returns None when not set
        assert_eq!(msg.content(), None);
        // Check internal state is None
        assert!(msg.role.is_none());
        assert!(msg.content.is_none());
    }

    #[test]
    fn test_set_role() {
        let msg = OllamaMessage2::new().set_role("user");
        assert_eq!(msg.role(), Some("user")); // Fixed: Compare with Some("user")
    }

    #[test]
    fn test_set_content() {
        let msg = OllamaMessage2::new().set_content("Hello");
        assert_eq!(msg.content(), Some("Hello"));
    }

    #[test]
    fn test_from_json_valid() {
        let json_data = json!({
            "role": "assistant",
            "content": "Hi there!"
        });
        let msg_result = OllamaMessage2::from_json(json_data);
        assert!(msg_result.is_ok());
        let msg = msg_result.unwrap();
        assert_eq!(msg.role(), Some("assistant")); // Fixed: Compare with Some("assistant")
        assert_eq!(msg.content(), Some("Hi there!"));
    }

    #[test]
    fn test_from_json_invalid_type() {
        let json_data = json!({
            "role": 123, // Invalid type
            "content": "Hi there!"
        });
        let msg_result = OllamaMessage2::from_json(json_data);
        assert!(msg_result.is_err());
    }

    #[test]
    fn test_from_json_missing_field() {
        let json_data = json!({
            "role": "user"
            // Missing content
        });
        // serde_json will deserialize missing optional fields as None
        let msg_result = OllamaMessage2::from_json(json_data);
        assert!(msg_result.is_ok());
        let msg = msg_result.unwrap();
        assert_eq!(msg.role(), Some("user")); // Fixed: Compare with Some("user")
        // Check getter returns None for missing field
        assert_eq!(msg.content(), None);
        // Check internal state is None
        assert!(msg.content.is_none());
    }

    #[test]
    fn test_to_json() {
        let msg = OllamaMessage2::new()
            .set_role("system")
            .set_content("You are a helpful assistant.");
        let json_val = msg.to_json();
        let expected_json = json!({
            "role": "system",
            "content": "You are a helpful assistant."
        });
        assert_eq!(json_val, expected_json);
    }

    #[test]
    fn test_to_json_empty() {
        let msg = OllamaMessage2::new();
        let json_val = msg.to_json();
        // Because of skip_serializing_if, empty fields should not be present
        let expected_json = json!({});
        assert_eq!(json_val, expected_json);
    }

    #[test]
    fn test_to_json_partial() {
        let msg = OllamaMessage2::new().set_role("user");
        let json_val = msg.to_json();
        let expected_json = json!({ "role": "user" });
        assert_eq!(json_val, expected_json);

        let msg = OllamaMessage2::new().set_content("test");
        let json_val = msg.to_json();
        let expected_json = json!({ "content": "test" });
        assert_eq!(json_val, expected_json);
    }
}

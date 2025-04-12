use serde_json::Value as JsonValue;
use serde_json::json;

// ===
// STRUCT: GeminiContent
// ===

/// Represents a content object in a Gemini API request.
///
/// This struct allows building and manipulating a single content object
/// within the request, including adding text and code parts.
pub struct GeminiContent {
    content: JsonValue,
}

// ===
// PUBLIC: GeminiContent
// ===

impl GeminiContent {
    /// Creates a new empty GeminiContent instance.
    ///
    /// # Returns
    /// * A new GeminiContent instance with an empty JSON content object
    pub fn new() -> Self {
        GeminiContent {
            content: JsonValue::default(),
        }
    }

    /// Creates a new GeminiContent with a text part.
    ///
    /// # Arguments
    /// * `text` - The text to add to the content
    ///
    /// # Returns
    /// * A new GeminiContent instance with the text added
    pub fn from_str(text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.add_text(text);
        content
    }

    /// Creates a new GeminiContent with a text part and sets the role.
    ///
    /// # Arguments
    /// * `role` - The role to set (e.g., "user", "system")
    /// * `text` - The text to add to the content
    ///
    /// # Returns
    /// * A new GeminiContent instance with the role set and the text added
    pub fn from_prompt(role: &str, text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.set_role(role).add_text(text);
        content
    }

    /// Creates a new GeminiContent with a text part and sets the role to "system".
    ///
    /// # Arguments
    /// * `text` - The text to add to the content
    ///
    /// # Returns
    /// * A new GeminiContent instance with the role set to "system" and the text added
    pub fn from_system_prompt(text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.set_role("system").add_text(text);
        content
    }

    /// Creates a new GeminiContent from an existing JSON value.
    ///
    /// # Arguments
    /// * `json_value` - The JSON value to create the content from
    ///
    /// # Returns
    /// * A new GeminiContent instance with the provided JSON content
    pub fn from_json(json_value: JsonValue) -> Self {
        GeminiContent {
            content: json_value,
        }
    }

    /// Returns a reference to the internal JSON object.
    ///
    /// # Returns
    /// * A reference to the JSON value representing the content
    pub fn as_json(&self) -> &JsonValue {
        &self.content
    }

    /// Consumes the GeminiContent and returns the internal JSON object.
    ///
    /// # Returns
    /// * The JSON value representing the content
    pub fn to_json(self) -> JsonValue {
        self.content
    }

    /// Sets the role for this content.
    ///
    /// # Arguments
    /// * `role` - The role to set (e.g., "user", "model", "system")
    ///
    /// # Returns
    /// * A mutable reference to self for method chaining
    pub fn set_role(&mut self, role: &str) -> &mut Self {
        self.content["role"] = JsonValue::String(role.to_string());
        self
    }

    /// Adds a text part to this content.
    ///
    /// # Arguments
    /// * `text` - The text to add
    ///
    /// # Returns
    /// * A mutable reference to self for method chaining
    pub fn add_text(&mut self, text: &str) -> &mut Self {
        let text_part = json!({
            "text": text
        });

        self.add_part(text_part)
    }

    /// Adds executable code as a part to this content.
    ///
    /// # Arguments
    /// * `language` - The programming language of the code
    /// * `code` - The code to add
    ///
    /// # Returns
    /// * A mutable reference to self for method chaining
    pub fn add_code(&mut self, language: &str, code: &str) -> &mut Self {
        let code_part = json!({
            "executable_code": {
                "language": language,
                "code": code
            }
        });

        self.add_part(code_part)
    }
}

// ===
// PRIVATE: GeminiContent
// ===

impl GeminiContent {
    /// Adds a part to the content's parts array.
    ///
    /// This private method handles adding any kind of part (text, code, etc.)
    /// to the content, creating the parts array if it doesn't exist yet.
    ///
    /// # Arguments
    /// * `part` - The JSON value representing the part to add
    ///
    /// # Returns
    /// * A mutable reference to self for method chaining
    fn add_part(&mut self, part: JsonValue) -> &mut Self {
        // Initialize with empty 'parts' array only if it doesn't already exist.
        if !self.content.get("parts").map_or(false, |p| p.is_array()) {
            self.content["parts"] = JsonValue::Array(vec![]);
        }

        // Add the new part to the 'parts' array.
        let parts = self.content["parts"].as_array_mut().unwrap();
        parts.push(part);

        self
    }
}

// ===
// TESTS: GeminiContent
// ===

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests creating a GeminiContent from a string.
    ///
    /// This test verifies that:
    /// - A GeminiContent can be created with text content
    /// - The text is properly structured in the JSON
    #[test]
    fn test_from_str() {
        let text = "Hello, world!";
        let content = GeminiContent::from_str(text);

        // Verify parts array exists and has one entry
        assert!(content.as_json()["parts"].is_array());
        assert_eq!(content.as_json()["parts"].as_array().unwrap().len(), 1);

        // Verify text content
        assert_eq!(content.as_json()["parts"][0]["text"], text);
    }

    /// Tests creating a GeminiContent with user role.
    ///
    /// This test verifies that:
    /// - A GeminiContent can be created with user role and text content
    /// - Both role and text are properly structured in the JSON
    #[test]
    fn test_from_user_prompt() {
        let text = "What is machine learning?";
        let content = GeminiContent::from_prompt("user", text);

        // Verify role is set to "user"
        assert_eq!(content.as_json()["role"], "user");

        // Verify parts array exists and has one entry with the correct text
        assert!(content.as_json()["parts"].is_array());
        assert_eq!(content.as_json()["parts"].as_array().unwrap().len(), 1);
        assert_eq!(content.as_json()["parts"][0]["text"], text);
    }

    /// Tests creating a GeminiContent with system role.
    ///
    /// This test verifies that:
    /// - A GeminiContent can be created with system role and text content
    /// - Both role and text are properly structured in the JSON
    #[test]
    fn test_from_system_prompt() {
        let text = "You are a helpful AI assistant";
        let content = GeminiContent::from_system_prompt(text);

        // Verify role is set to "system"
        assert_eq!(content.as_json()["role"], "system");

        // Verify parts array exists and has one entry with the correct text
        assert!(content.as_json()["parts"].is_array());
        assert_eq!(content.as_json()["parts"].as_array().unwrap().len(), 1);
        assert_eq!(content.as_json()["parts"][0]["text"], text);
    }

    /// Tests creating a GeminiContent from a JSON value.
    ///
    /// This test verifies that:
    /// - A GeminiContent can be created from an existing JSON value
    /// - The original JSON structure is maintained
    #[test]
    fn test_from_json() {
        let json = json!({
            "role": "model",
            "parts": [
                {"text": "This is a test response"}
            ]
        });

        let content = GeminiContent::from_json(json.clone());

        // Verify the content matches the original JSON
        assert_eq!(content.as_json(), &json);
    }

    /// Tests the as_json method.
    ///
    /// This test verifies that:
    /// - as_json returns a reference to the internal JSON value
    /// - The reference can be used to access JSON properties
    #[test]
    fn test_as_json() {
        let mut content = GeminiContent::new();
        content.set_role("assistant").add_text("Hello");

        let json_ref = content.as_json();

        // Verify we can access properties through the reference
        assert_eq!(json_ref["role"], "assistant");
        assert_eq!(json_ref["parts"][0]["text"], "Hello");
    }

    /// Tests the to_json method.
    ///
    /// This test verifies that:
    /// - to_json consumes the GeminiContent and returns the internal JSON value
    /// - The returned JSON value contains all expected properties
    #[test]
    fn test_to_json() {
        let mut content = GeminiContent::new();
        content.set_role("assistant").add_text("Hello");

        let json = content.to_json();

        // Verify the JSON structure
        assert_eq!(json["role"], "assistant");
        assert_eq!(json["parts"][0]["text"], "Hello");
    }

    /// Tests the set_role method.
    ///
    /// This test verifies that:
    /// - A role can be set on a GeminiContent instance
    /// - The method returns self for chaining
    /// - Multiple calls update the role as expected
    #[test]
    fn test_set_role() {
        let mut content = GeminiContent::new();

        // Set initial role and verify
        content.set_role("user");
        assert_eq!(content.as_json()["role"], "user");

        // Change role and verify
        content.set_role("assistant");
        assert_eq!(content.as_json()["role"], "assistant");
    }

    /// Tests the add_text method.
    ///
    /// This test verifies that:
    /// - Text content can be added to a GeminiContent instance
    /// - The method returns self for chaining
    /// - Multiple calls add multiple text parts
    #[test]
    fn test_add_text() {
        let mut content = GeminiContent::new();

        // Add first text part and verify
        content.add_text("First message");
        assert_eq!(content.as_json()["parts"][0]["text"], "First message");

        // Add second text part and verify both exist
        content.add_text("Second message");
        assert_eq!(content.as_json()["parts"][0]["text"], "First message");
        assert_eq!(content.as_json()["parts"][1]["text"], "Second message");
        assert_eq!(content.as_json()["parts"].as_array().unwrap().len(), 2);
    }

    /// Tests the add_code method.
    ///
    /// This test verifies that:
    /// - Executable code can be added to a GeminiContent instance
    /// - The method returns self for chaining
    /// - Code parts have the correct structure with language and code content
    #[test]
    fn test_add_code() {
        let mut content = GeminiContent::new();

        // Add a code part
        content.add_code("python", "print('Hello, world!')");

        // Verify the code part structure
        let code_part = &content.as_json()["parts"][0]["executable_code"];
        assert_eq!(code_part["language"], "python");
        assert_eq!(code_part["code"], "print('Hello, world!')");
    }

    /// Tests method chaining with multiple operations.
    ///
    /// This test verifies that:
    /// - Multiple methods can be chained together
    /// - The resulting JSON structure contains all expected elements
    #[test]
    fn test_method_chaining() {
        let mut content = GeminiContent::new();
        content
            .set_role("user")
            .add_text("Please run this code:")
            .add_code("rust", "fn main() {\n    println!(\"Hello, Rust!\");\n}");

        let content = content.to_json();

        // Verify role
        assert_eq!(content["role"], "user");

        // Verify parts array has two entries
        assert_eq!(content["parts"].as_array().unwrap().len(), 2);

        // Verify text part
        assert_eq!(content["parts"][0]["text"], "Please run this code:");

        // Verify code part
        assert_eq!(content["parts"][1]["executable_code"]["language"], "rust");
        assert_eq!(
            content["parts"][1]["executable_code"]["code"],
            "fn main() {\n    println!(\"Hello, Rust!\");\n}"
        );
    }

    /// Tests adding multiple different types of parts.
    ///
    /// This test verifies that:
    /// - A mix of text and code parts can be added
    /// - All parts are properly stored in the correct order
    #[test]
    fn test_mixed_content() {
        let mut content = GeminiContent::new();
        content
            .set_role("user")
            .add_text("Here's a Python example:")
            .add_code("python", "print('Example 1')")
            .add_text("And a Rust example:")
            .add_code("rust", "println!(\"Example 2\");");

        let parts = content.as_json()["parts"].as_array().unwrap();

        // Verify we have 4 parts in the correct order
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0]["text"], "Here's a Python example:");
        assert_eq!(parts[1]["executable_code"]["language"], "python");
        assert_eq!(parts[1]["executable_code"]["code"], "print('Example 1')");
        assert_eq!(parts[2]["text"], "And a Rust example:");
        assert_eq!(parts[3]["executable_code"]["language"], "rust");
        assert_eq!(
            parts[3]["executable_code"]["code"],
            "println!(\"Example 2\");"
        );
    }
}

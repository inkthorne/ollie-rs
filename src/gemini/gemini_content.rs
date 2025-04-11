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
    /// Creates a new GeminiContent wrapper around a JSON content object.
    ///
    /// # Returns
    /// * A new empty GeminiContent instance
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
    pub fn text(text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.add_text(text);
        content
    }

    /// Creates a new GeminiContent with a text part and sets the role to "user".
    ///
    /// # Arguments
    /// * `text` - The text to add to the content
    ///
    /// # Returns
    /// * A new GeminiContent instance with the role set to "user" and the text added
    pub fn user(text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.set_role("user").add_text(text);
        content
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

use crate::GeminiContent;
use serde_json::Value as JsonValue;

// ===
// ENUM: GeminiRole
// ===

/// Represents the role of a content part in a Gemini API request.
///
/// The role defines who or what is responsible for a particular content part.
/// Gemini supports system, user, and tool roles.
#[derive(Debug, Clone, PartialEq)]
pub enum GeminiRole {
    System,
    User,
    Tool,
}

impl GeminiRole {
    /// Converts the role to its string representation for the API.
    ///
    /// # Returns
    /// * String representation of the role
    pub fn as_str(&self) -> &'static str {
        match self {
            GeminiRole::System => "system",
            GeminiRole::User => "user",
            GeminiRole::Tool => "tool",
        }
    }

    /// Creates a GeminiRole from a string.
    ///
    /// # Arguments
    /// * `role` - String representation of the role
    ///
    /// # Returns
    /// * The corresponding GeminiRole, or None if the string doesn't match
    pub fn from_str(role: &str) -> Option<Self> {
        match role.to_lowercase().as_str() {
            "system" => Some(GeminiRole::System),
            "user" => Some(GeminiRole::User),
            "tool" => Some(GeminiRole::Tool),
            _ => None,
        }
    }
}

// ===
// STRUCT: GeminiRequest
// ===

/// Represents a request to the Gemini AI model.
///
/// This struct allows building a complete request object with
/// model selection and content parts.
pub struct GeminiRequest {
    request: JsonValue,
}

// ===
// PUBLIC: GeminiRequest
// ===

impl GeminiRequest {
    /// Creates a new empty GeminiRequest.
    ///
    /// # Returns
    /// * A new GeminiRequest instance
    pub fn new() -> Self {
        GeminiRequest {
            request: JsonValue::default(),
        }
    }

    /// Creates a new GeminiRequest with a text input.
    ///
    /// This is a convenience method that creates a request with a single content
    /// object containing the provided text.
    ///
    /// # Arguments
    /// * `text` - The text to include in the request
    ///
    /// # Returns
    /// * A new GeminiRequest instance with the text added
    pub fn from_str(text: &str) -> Self {
        let content = GeminiContent::from_str(text);
        let mut request = GeminiRequest::new();
        request.add_content(content.to_json());
        request
    }

    /// Creates a new GeminiRequest with a text input and specified role.
    ///
    /// This is a convenience method that creates a request with a single content
    /// object containing the provided text with the specified role.
    ///
    /// # Arguments
    /// * `role` - The role to use for the content (system, user, or tool)
    /// * `text` - The text to include in the request
    ///
    /// # Returns
    /// * A new GeminiRequest instance with the text added with the specified role
    pub fn from_prompt(role: GeminiRole, text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.set_role(role.as_str());
        content.add_text(text);

        let mut request = GeminiRequest::new();
        request.add_content(content.to_json());
        request
    }

    /// Creates a new GeminiRequest from an existing JSON value.
    ///
    /// This is useful when you already have a JsonValue that you want to use as a request.
    ///
    /// # Arguments
    /// * `json` - The JsonValue to use as the request
    ///
    /// # Returns
    /// * A new GeminiRequest instance with the provided JSON
    pub fn from_json(json: JsonValue) -> Self {
        GeminiRequest {
            request: json.clone(),
        }
    }

    /// Returns the internal JSON object.
    ///
    /// # Returns
    /// * A reference to the JSON object representing the request
    pub fn as_json(&self) -> &JsonValue {
        &self.request
    }

    /// Consumes the request and returns the internal JSON object.
    ///
    /// # Returns
    /// * The JSON object representing the request
    pub fn to_json(self) -> JsonValue {
        self.request
    }

    /// Converts the request to a pretty-printed JSON string.
    ///
    /// # Returns
    /// * A formatted JSON string representation of the request
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.request).unwrap_or_default()
    }

    /// Sets the model to use for this request.
    ///
    /// # Arguments
    /// * `model` - The name of the model to use
    ///
    /// # Returns
    /// * A mutable reference to self for method chaining
    pub fn set_model(&mut self, model: &str) -> &mut Self {
        self.request["model"] = JsonValue::String(model.to_string());
        self
    }

    /// Adds a new content object to the request and returns a builder for it.
    ///
    /// This method creates a new content object in the request and returns
    /// a GeminiContent builder that can be used to construct its parts.
    ///
    /// # Returns
    /// * A GeminiContent builder for the new content object
    pub fn add_content(&mut self, content: JsonValue) -> &mut Self {
        // Ensure 'contents' is an array.
        if !self.request.get("contents").map_or(false, |c| c.is_array()) {
            self.request["contents"] = JsonValue::Array(vec![]);
        }

        // Add new 'content' to the array.
        self.request["contents"]
            .as_array_mut()
            .unwrap()
            .push(content);

        self
    }
}

// ===
// TESTS: GeminiRequest
// ===

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the creation of a GeminiRequest with code content.
    ///
    /// This test verifies that:
    /// - A model can be set correctly
    /// - Content with a role can be added
    /// - Text parts can be added to content
    /// - Executable code parts can be added to content
    /// - All parts are properly structured in the JSON output
    #[test]
    fn test_gemini_request_code() {
        let mut content = GeminiContent::new();
        content
            .set_role("user")
            .add_text("Can you tell me the result of executing this code?")
            .add_code("python", "print('Hello, world!')");

        let mut request = GeminiRequest::new();
        request
            .set_model("gpt-3.5-turbo")
            .add_content(content.to_json());

        let output = request.to_string_pretty();
        println!("GeminiRequest: {}", output);

        // Verify model is set correctly
        assert_eq!(
            request.as_json()["model"],
            JsonValue::String("gpt-3.5-turbo".to_string())
        );

        // Verify contents array has one item
        let contents = request.as_json()["contents"].as_array().unwrap();
        assert_eq!(contents.len(), 1);

        // Verify content properties
        let content_obj = &contents[0];
        assert_eq!(content_obj["role"], JsonValue::String("user".to_string()));

        // Verify parts array has two items
        let parts = content_obj["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 2);

        // Verify text part
        assert_eq!(
            parts[0]["text"],
            JsonValue::String("Can you tell me the result of executing this code?".to_string())
        );

        // Verify code part
        assert_eq!(
            parts[1]["executable_code"]["language"],
            JsonValue::String("python".to_string())
        );
        assert_eq!(
            parts[1]["executable_code"]["code"],
            JsonValue::String("print('Hello, world!')".to_string())
        );
    }
}

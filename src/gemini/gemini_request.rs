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
    pub fn text(text: &str) -> Self {
        let content = GeminiContent::text(text);
        let mut request = GeminiRequest::new();
        request.add_content(content.to_json());
        request
    }

    /// Returns the internal JSON object.
    ///
    /// # Returns
    /// * A reference to the JSON object representing the request
    pub fn as_json(&self) -> &JsonValue {
        &self.request
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

/*

// Preferred usage example:

let mut request = GeminiRequest::new();
request.set_model("gpt-3.5-turbo");

let mut content = request.add_content();
content
    .set_role("user")
    .add_text("Can you tell me the result of executing this code?")
    .add_code(code);



// Example code to be executed:

let mut request = GeminiTextRequest::new("How are you today?");
request.set_model("gpt-3.5-turbo");

*/

use serde_json::Value as JsonValue;
use serde_json::json;

// ===
// STRUCT: GeminiContent
// ===

/// Represents a content object in a Gemini API request.
///
/// This struct allows building and manipulating a single content object
/// within the request, including adding text and code parts.
pub struct GeminiContent<'a> {
    content: &'a mut JsonValue,
}

// ===
// PUBLIC: GeminiContent
// ===

impl<'a> GeminiContent<'a> {
    /// Creates a new GeminiContent wrapper around a JSON content object.
    ///
    /// Initializes the 'parts' array if it doesn't already exist.
    ///
    /// # Arguments
    /// * `content` - Mutable reference to a JSON object to be used as content
    pub fn new(content: &'a mut JsonValue) -> Self {
        // Initialize with empty 'parts' array only if it doesn't already exist.
        if !content.get("parts").map_or(false, |p| p.is_array()) {
            content["parts"] = JsonValue::Array(vec![]);
        }
        GeminiContent { content }
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

        self.content["parts"]
            .as_array_mut()
            .unwrap()
            .push(text_part);
        self
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

        self.content["parts"]
            .as_array_mut()
            .unwrap()
            .push(code_part);
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
    pub fn add_content(&mut self) -> GeminiContent {
        // Ensure 'contents' is an array.
        if !self.request.get("contents").map_or(false, |c| c.is_array()) {
            self.request["contents"] = JsonValue::Array(vec![]);
        }

        // Add new empty object to the array.
        let empty_content = json!({});
        self.request["contents"]
            .as_array_mut()
            .unwrap()
            .push(empty_content);

        // Get a mutable reference to the last element.
        let content_idx = self.request["contents"].as_array().unwrap().len() - 1;
        let content_ref = &mut self.request["contents"][content_idx];

        // Return a GeminiContent that references the new content object.
        GeminiContent::new(content_ref)
    }
}

// ===
// STRUCT: GeminiTextRequest
// ===

/// A simplified request builder for sending text prompts to Gemini.
///
/// This struct provides a convenient way to create a simple text-based
/// request without manually building the content structure.
pub struct GeminiTextRequest {
    request: GeminiRequest,
}

// ===
// PUBLIC: GeminiTextRequest
// ===

impl GeminiTextRequest {
    /// Creates a new GeminiTextRequest with the provided text prompt.
    ///
    /// # Arguments
    /// * `text` - The text prompt to send to the model
    ///
    /// # Returns
    /// * A new GeminiTextRequest instance
    pub fn new(text: &str) -> Self {
        let mut request = GeminiTextRequest {
            request: GeminiRequest::new(),
        };
        request.set_text(text);
        request
    }

    /// Returns the internal JSON object.
    ///
    /// # Returns
    /// * A reference to the JSON object representing the request
    pub fn as_json(&self) -> &JsonValue {
        self.request.as_json()
    }

    /// Converts the request to a pretty-printed JSON string.
    ///
    /// # Returns
    /// * A formatted JSON string representation of the request
    pub fn to_string_pretty(&self) -> String {
        self.request.to_string_pretty()
    }
}

// ===
// PRIVATE: GeminiTextRequest
// ===

impl GeminiTextRequest {
    /// Sets the text content for this request.
    ///
    /// # Arguments
    /// * `text` - The text content to add
    ///
    /// # Returns
    /// * A mutable reference to self for method chaining
    fn set_text(&mut self, text: &str) -> &mut Self {
        let mut content = self.request.add_content();
        content.add_text(text);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===
    // TESTS: GeminiRequest
    // ===

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
        let mut request = GeminiRequest::new();
        request.set_model("gpt-3.5-turbo");

        let mut content = request.add_content();
        content
            .set_role("user")
            .add_text("Can you tell me the result of executing this code?")
            .add_code("python", "print('Hello, world!')");

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

    // ===
    // TESTS: GeminiTextRequest
    // ===

    /// Tests the creation of a simplified GeminiTextRequest.
    ///
    /// This test verifies that:
    /// - A text request can be created with a prompt
    /// - The default model is set correctly
    /// - Contents array is properly initialized
    /// - The text content is correctly added to the request
    #[test]
    fn test_gemini_text_request() {
        let request = GeminiTextRequest::new("How are you today?");

        // Print the request for debugging
        let output = request.to_string_pretty();
        println!("GeminiTextRequest: {}", output);

        // Use the new as_json method directly
        let json_request = request.as_json();

        // Verify model is set correctly
        assert_eq!(
            json_request["model"],
            JsonValue::String("gpt-3.5-turbo".to_string())
        );

        // Verify contents array has one item
        let contents = json_request["contents"].as_array().unwrap();
        assert_eq!(contents.len(), 1);

        // Verify the text content was added correctly
        let parts = contents[0]["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(
            parts[0]["text"],
            JsonValue::String("How are you today?".to_string())
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

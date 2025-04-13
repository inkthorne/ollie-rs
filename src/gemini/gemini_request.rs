use crate::{GeminiContent, GeminiResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

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

/// Represents a request to the Gemini API.
///
/// Contains a collection of content parts that make up the conversation or prompt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
}

// ===
// PUBLIC: GeminiRequest
// ===

impl GeminiRequest {
    /// Creates a new empty GeminiRequest.
    ///
    /// # Returns
    /// * A new GeminiRequest with no content
    pub fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    /// Creates a GeminiRequest from a prompt with a specified role.
    ///
    /// # Arguments
    /// * `role` - The role of the content (system, user, or tool)
    /// * `text` - The text content of the prompt
    ///
    /// # Returns
    /// * A new GeminiRequest containing the prompt
    pub fn from_prompt(role: GeminiRole, text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.set_role(role.as_str());
        content.add_text(text);

        let mut request = GeminiRequest::new();
        request.add_content(content);
        request
    }

    /// Creates a GeminiRequest from a text string.
    ///
    /// # Arguments
    /// * `text` - The text content to include in the request
    ///
    /// # Returns
    /// * A new GeminiRequest containing the text
    pub fn from_str(text: &str) -> Self {
        let mut content = GeminiContent::new();
        content.add_text(text);

        let mut request = GeminiRequest::new();
        request.add_content(content);
        request
    }

    /// Converts the request to a JSON value.
    ///
    /// # Returns
    /// * JsonValue representation of the request
    pub fn to_json(&self) -> JsonValue {
        serde_json::to_value(&self).unwrap_or_default()
    }

    /// Converts the request to a pretty-printed JSON string.
    ///
    /// # Returns
    /// * A formatted JSON string representing the request
    pub fn to_string_pretty(&self) -> String {
        let request_json = self.to_json();
        serde_json::to_string_pretty(&request_json).unwrap_or_default()
    }

    /// Adds a content part to the request.
    ///
    /// # Arguments
    /// * `content` - The GeminiContent to add
    ///
    /// # Returns
    /// * &mut Self for method chaining
    pub fn add_content(&mut self, content: GeminiContent) -> &mut Self {
        self.contents.push(content);
        self
    }

    /// Adds a prompt with a specified role to the request.
    ///
    /// # Arguments
    /// * `role` - The role of the content (system, user, or tool)
    /// * `text` - The text content of the prompt
    ///
    /// # Returns
    /// * &mut Self for method chaining
    pub fn add_prompt(&mut self, role: GeminiRole, text: &str) -> &mut Self {
        let mut content = GeminiContent::new();
        content.set_role(role.as_str()).add_text(text);
        self.add_content(content)
    }

    /// Adds a response content to the request.
    ///
    /// This is useful for building conversation history.
    ///
    /// # Arguments
    /// * `response` - The GeminiResponse to add to the request
    ///
    /// # Returns
    /// * &mut Self for method chaining
    pub fn add_response(&mut self, response: &GeminiResponse) -> &mut Self {
        if let Some(content) = response.content() {
            self.add_content(content.clone());
        }
        self
    }
}

// ===
// TRAIT: GeminiRequest (fmt::Display)
// ===

impl fmt::Display for GeminiRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_pretty())
    }
}

// ===
// TESTS: GeminiRequest
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeminiPart;

    #[test]
    fn test_gemini_request_new() {
        let request = GeminiRequest::new();
        assert!(request.contents.is_empty());
    }

    #[test]
    fn test_gemini_request_from_prompt() {
        let request = GeminiRequest::from_prompt(GeminiRole::User, "Hello, Gemini!");
        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.contents[0].role, Some("user".to_string()));

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "Hello, Gemini!");
        } else {
            panic!("Expected text part");
        }
    }

    #[test]
    fn test_gemini_request_from_str() {
        let request = GeminiRequest::from_str("Simple text prompt");
        assert_eq!(request.contents.len(), 1);

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "Simple text prompt");
        } else {
            panic!("Expected text part");
        }
    }

    #[test]
    fn test_gemini_request_add_content() {
        let mut request = GeminiRequest::new();
        let mut content1 = GeminiContent::new();
        content1.set_role("user").add_text("First message");

        let mut content2 = GeminiContent::new();
        content2.set_role("system").add_text("Second message");

        request.add_content(content1).add_content(content2);

        assert_eq!(request.contents.len(), 2);
        assert_eq!(request.contents[0].role, Some("user".to_string()));
        assert_eq!(request.contents[1].role, Some("system".to_string()));
    }

    #[test]
    fn test_gemini_request_add_prompt() {
        let mut request = GeminiRequest::new();
        request
            .add_prompt(GeminiRole::System, "You are a helpful assistant")
            .add_prompt(GeminiRole::User, "Tell me about Rust");

        assert_eq!(request.contents.len(), 2);
        assert_eq!(request.contents[0].role, Some("system".to_string()));
        assert_eq!(request.contents[1].role, Some("user".to_string()));

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "You are a helpful assistant");
        } else {
            panic!("Expected text part");
        }

        if let GeminiPart::Text(text_part) = &request.contents[1].parts[0] {
            assert_eq!(text_part.text, "Tell me about Rust");
        } else {
            panic!("Expected text part");
        }
    }

    #[test]
    fn test_gemini_request_to_json() {
        let mut request = GeminiRequest::new();
        let mut content = GeminiContent::new();
        content.set_role("user").add_text("Convert this to JSON");
        request.add_content(content);

        let json = request.to_json();
        assert!(json.is_object());

        let contents = json
            .as_object()
            .unwrap()
            .get("contents")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(contents.len(), 1);
    }

    #[test]
    fn test_gemini_request_to_string_pretty() {
        let request = GeminiRequest::from_prompt(GeminiRole::User, "Hello");
        let json_str = request.to_string_pretty();

        // Verify it's valid JSON
        assert!(serde_json::from_str::<JsonValue>(&json_str).is_ok());
        assert!(json_str.contains("contents"));
        assert!(json_str.contains("Hello"));
    }

    #[test]
    fn test_gemini_request_display() {
        let request = GeminiRequest::from_prompt(GeminiRole::User, "Test display");
        let display_str = format!("{}", request);

        // Display should use to_string_pretty()
        let pretty_str = request.to_string_pretty();
        assert_eq!(display_str, pretty_str);
    }

    #[test]
    fn test_gemini_role_as_str() {
        assert_eq!(GeminiRole::System.as_str(), "system");
        assert_eq!(GeminiRole::User.as_str(), "user");
        assert_eq!(GeminiRole::Tool.as_str(), "tool");
    }

    #[test]
    fn test_gemini_role_from_str() {
        assert_eq!(GeminiRole::from_str("system"), Some(GeminiRole::System));
        assert_eq!(GeminiRole::from_str("USER"), Some(GeminiRole::User));
        assert_eq!(GeminiRole::from_str("Tool"), Some(GeminiRole::Tool));
        assert_eq!(GeminiRole::from_str("unknown"), None);
    }

    #[test]
    fn test_gemini_request_add_response() {
        use crate::GeminiCandidate;

        let mut request = GeminiRequest::new();
        request.add_prompt(GeminiRole::User, "Initial prompt");

        // Create a mock GeminiContent for the response
        let mut response_content = GeminiContent::new();
        response_content.set_role("model").add_text("Response text");

        // Create a mock GeminiCandidate
        let candidate = GeminiCandidate {
            content: response_content,
            finish_reason: None,
            index: Some(0),
        };

        // Create the response with the candidate
        let response = GeminiResponse {
            candidates: vec![candidate],
        };

        // Test adding the response to the request
        request.add_response(&response);

        // Verify it was added correctly
        assert_eq!(request.contents.len(), 2);
        assert_eq!(request.contents[1].role, Some("model".to_string()));

        if let GeminiPart::Text(text_part) = &request.contents[1].parts[0] {
            assert_eq!(text_part.text, "Response text");
        } else {
            panic!("Expected text part");
        }
    }
}

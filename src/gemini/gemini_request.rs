use crate::GeminiFunctionResponse;
use crate::GeminiPart;
use crate::GeminiPrompt;
use crate::GeminiRole;
use crate::GeminiToolDeclaration;
use crate::{GeminiContent, GeminiResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

// ===
// STRUCT: GeminiRequest
// ===

/// Represents a request to the Gemini API.
///
/// Contains a collection of content parts that make up the conversation or prompt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<GeminiContent>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<GeminiToolDeclaration>,
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
            tools: Vec::new(),
        }
    }

    /// Creates a GeminiRequest from a prompt with a specified role.
    ///
    /// # Arguments
    /// * `prompt` - The GeminiPrompt containing the role and text content
    ///
    /// # Returns
    /// * A new GeminiRequest containing the prompt
    pub fn from_prompt(prompt: &GeminiPrompt) -> Self {
        let mut content = GeminiContent::new();

        if let Some(role) = &prompt.role {
            content.set_role(*role);
        }

        content.add_text(prompt.text.as_str());

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

    /// Adds a function response as a content part to the request.
    ///
    /// # Arguments
    /// * `function_response` - The GeminiFunctionResponse to add as a tool content part
    ///
    /// # Returns
    /// * &mut Self for method chaining
    pub fn add_function_response(
        &mut self,
        function_response: GeminiFunctionResponse,
    ) -> &mut Self {
        let mut content = GeminiContent::new();
        content.set_role(GeminiRole::Tool);
        content.add_part(GeminiPart::FunctionResponse(function_response));

        self.add_content(content);
        self
    }

    /// Adds a prompt with a specified role to the request.
    ///
    /// # Arguments
    /// * `prompt` - The GeminiPrompt containing the role and text content
    ///
    /// # Returns
    /// * &mut Self for method chaining
    pub fn add_prompt(&mut self, prompt: &GeminiPrompt) -> &mut Self {
        let mut content = GeminiContent::new();

        if let Some(role) = &prompt.role {
            content.set_role(*role);
        }

        content.add_text(prompt.text.as_str());
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

    /// Adds a tool declaration to the request.
    ///
    /// # Arguments
    /// * `tool` - The GeminiToolDeclaration to add
    ///
    /// # Returns
    /// * &mut Self for method chaining
    pub fn add_tool(&mut self, tool: GeminiToolDeclaration) -> &mut Self {
        self.tools.push(tool);
        self
    }
}

// ===
// TRAIT: GeminiRequest (fmt::Display)
// ===

/// Implementation of Display trait for GeminiRequest
///
/// Formats the GeminiRequest as a pretty-printed JSON string
impl fmt::Display for GeminiRequest {
    /// Formats the GeminiRequest as a pretty-printed JSON string
    ///
    /// # Arguments
    /// * `f` - The formatter to write to
    ///
    /// # Returns
    /// * fmt::Result indicating whether the operation succeeded or failed
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_pretty())
    }
}

// ===
// TRAIT: GeminiRequest (From<&str>)
// ===

/// Implementation of From<&str> trait for GeminiRequest
///
/// Allows conversion from a string slice to a GeminiRequest
impl From<&str> for GeminiRequest {
    /// Creates a GeminiRequest from a string slice.
    ///
    /// # Arguments
    /// * `s` - The text content to include in the request
    ///
    /// # Returns
    /// * A new GeminiRequest containing the text
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

// ===
// TRAIT: GeminiRequest (From<String>)
// ===

/// Implementation of From<String> trait for GeminiRequest
///
/// Allows conversion from a String to a GeminiRequest
impl From<String> for GeminiRequest {
    /// Creates a GeminiRequest from a String.
    ///
    /// # Arguments
    /// * `s` - The String content to include in the request
    ///
    /// # Returns
    /// * A new GeminiRequest containing the text
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

// ===
// TRAIT: GeminiRequest (From<GeminiPrompt>)
// ===

/// Implementation of From<GeminiPrompt> trait for GeminiRequest
///
/// Allows conversion from a GeminiPrompt to a GeminiRequest
impl From<GeminiPrompt> for GeminiRequest {
    /// Creates a GeminiRequest from a GeminiPrompt.
    ///
    /// # Arguments
    /// * `prompt` - The GeminiPrompt to convert to a request
    ///
    /// # Returns
    /// * A new GeminiRequest containing the prompt content
    fn from(prompt: GeminiPrompt) -> Self {
        let mut request = GeminiRequest::new();
        let mut content = GeminiContent::new();

        if let Some(role) = &prompt.role {
            content.set_role(*role);
        }

        content.add_text(&prompt.text);
        request.add_content(content);
        request
    }
}

// ===
// TESTS: GeminiRequest
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeminiPart;
    use crate::GeminiPromptSystem;
    use crate::GeminiPromptUser;
    use crate::GeminiRole;

    #[test]
    fn test_gemini_request_new() {
        let request = GeminiRequest::new();
        assert!(request.contents.is_empty());
    }

    #[test]
    fn test_gemini_request_from_prompt() {
        let prompt = GeminiPromptUser::new("Hello, Gemini!");
        let request = GeminiRequest::from_prompt(&prompt);
        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.contents[0].role(), Some(GeminiRole::User));

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
    fn test_gemini_request_from_trait() {
        // Test the From<&str> trait implementation
        let request: GeminiRequest = "Convert me to a request".into();
        assert_eq!(request.contents.len(), 1);

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "Convert me to a request");
        } else {
            panic!("Expected text part");
        }

        // Test using From::from explicitly
        let request = GeminiRequest::from("Another test");
        assert_eq!(request.contents.len(), 1);

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "Another test");
        } else {
            panic!("Expected text part");
        }

        // Test the From<String> trait implementation
        let request: GeminiRequest = "Convert me to a request".to_string().into();
        assert_eq!(request.contents.len(), 1);

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "Convert me to a request");
        } else {
            panic!("Expected text part");
        }

        // Test using From::from explicitly with String
        let request = GeminiRequest::from("Another test".to_string());
        assert_eq!(request.contents.len(), 1);

        if let GeminiPart::Text(text_part) = &request.contents[0].parts[0] {
            assert_eq!(text_part.text, "Another test");
        } else {
            panic!("Expected text part");
        }
    }

    #[test]
    fn test_gemini_request_add_content() {
        let mut request = GeminiRequest::new();
        let mut content1 = GeminiContent::new();
        content1
            .set_role(GeminiRole::User)
            .add_text("First message");

        let mut content2 = GeminiContent::new();
        content2
            .set_role(GeminiRole::System)
            .add_text("Second message");

        request.add_content(content1).add_content(content2);

        assert_eq!(request.contents.len(), 2);
        assert_eq!(request.contents[0].role(), Some(GeminiRole::User));
        assert_eq!(request.contents[1].role(), Some(GeminiRole::System));
    }

    #[test]
    fn test_gemini_request_add_prompt() {
        let mut request = GeminiRequest::new();
        request
            .add_prompt(&GeminiPromptSystem::new("You are a helpful assistant"))
            .add_prompt(&GeminiPromptUser::new("Tell me about Rust"));

        assert_eq!(request.contents.len(), 2);
        assert_eq!(request.contents[0].role(), Some(GeminiRole::System));
        assert_eq!(request.contents[1].role(), Some(GeminiRole::User));

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
        content
            .set_role(GeminiRole::User)
            .add_text("Convert this to JSON");
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
        let request = GeminiRequest::from_prompt(&GeminiPromptUser::new("Hello"));
        let json_str = request.to_string_pretty();

        // Verify it's valid JSON
        assert!(serde_json::from_str::<JsonValue>(&json_str).is_ok());
        assert!(json_str.contains("contents"));
        assert!(json_str.contains("Hello"));
    }

    #[test]
    fn test_gemini_request_display() {
        let request = GeminiRequest::from_prompt(&GeminiPromptUser::new("Test display"));
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
        let prompt = GeminiPromptUser::new("Initial prompt");
        request.add_prompt(&prompt);

        // Create a mock GeminiContent for the response
        let mut response_content = GeminiContent::new();
        response_content
            .set_role(GeminiRole::System)
            .add_text("Response text");

        // Create a mock GeminiCandidate
        let candidate = GeminiCandidate {
            content: response_content,
            finish_reason: None,
            index: Some(0),
        };

        // Create the response with the candidate
        let response = GeminiResponse {
            candidates: Some(vec![candidate]),
            error: None,
        };

        // Test adding the response to the request
        request.add_response(&response);

        // Verify it was added correctly
        assert_eq!(request.contents.len(), 2);
        assert_eq!(request.contents[1].role(), Some(GeminiRole::System));

        if let GeminiPart::Text(text_part) = &request.contents[1].parts[0] {
            assert_eq!(text_part.text, "Response text");
        } else {
            panic!("Expected text part");
        }
    }
}

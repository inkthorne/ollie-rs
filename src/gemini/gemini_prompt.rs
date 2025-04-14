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

// ===
// PUBLIC: GeminiRole
// ===

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
// STRUCT: GeminiPrompt
// ===

/// Represents a prompt for the Gemini API with optional role assignment.
///
/// A prompt consists of text content and an optional role (user, system, or tool)
/// that determines how the model interprets the content.
#[derive(Debug, Clone)]
pub struct GeminiPrompt {
    pub role: Option<GeminiRole>,
    pub text: String,
}

impl GeminiPrompt {
    /// Creates a new GeminiPrompt instance with the specified text and no role assigned.
    ///
    /// # Arguments
    /// * `text` - The content of the prompt
    ///
    /// # Returns
    /// * A new GeminiPrompt with no role and the provided text
    pub fn new(text: &str) -> Self {
        GeminiPrompt {
            role: None,
            text: text.to_string(),
        }
    }
}

// ===
// STRUCT: GeminiPromptSystem
// ===

pub struct GeminiPromptSystem;

impl GeminiPromptSystem {
    /// Creates a new GeminiPrompt instance with the System role.
    ///
    /// # Arguments
    /// * `text` - The content of the system prompt
    ///
    /// # Returns
    /// * A GeminiPrompt with the System role and provided text
    pub fn new(text: &str) -> GeminiPrompt {
        GeminiPrompt {
            role: Some(GeminiRole::System),
            text: text.to_string(),
        }
    }
}

// ===
// STRUCT: GeminiPromptTool
// ===

pub struct GeminiPromptTool;

impl GeminiPromptTool {
    /// Creates a new GeminiPrompt instance with the Tool role.
    ///
    /// # Arguments
    /// * `text` - The content of the tool prompt
    ///
    /// # Returns
    /// * A GeminiPrompt with the Tool role and provided text
    pub fn new(text: &str) -> GeminiPrompt {
        GeminiPrompt {
            role: Some(GeminiRole::Tool),
            text: text.to_string(),
        }
    }
}

// ===
// STRUCT: GeminiPromptUser
// ===

pub struct GeminiPromptUser;

impl GeminiPromptUser {
    /// Creates a new GeminiPrompt instance with the User role.
    ///
    /// # Arguments
    /// * `text` - The content of the user prompt
    ///
    /// # Returns
    /// * A GeminiPrompt with the User role and provided text
    pub fn new(text: &str) -> GeminiPrompt {
        GeminiPrompt {
            role: Some(GeminiRole::User),
            text: text.to_string(),
        }
    }
}

// ===
// TESTS: GeminiPrompt
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeminiPart;
    use crate::GeminiRequest;

    #[test]
    fn test_gemini_prompt_new() {
        let prompt = GeminiPrompt::new("Test prompt text");
        assert_eq!(prompt.text, "Test prompt text");
        assert_eq!(prompt.role, None);
    }

    #[test]
    fn test_gemini_prompt_user_new() {
        let prompt = GeminiPromptUser::new("User prompt");
        assert_eq!(prompt.text, "User prompt");
        assert_eq!(prompt.role, Some(GeminiRole::User));
    }

    #[test]
    fn test_gemini_prompt_system_new() {
        let prompt = GeminiPromptSystem::new("System instructions");
        assert_eq!(prompt.text, "System instructions");
        assert_eq!(prompt.role, Some(GeminiRole::System));
    }

    #[test]
    fn test_gemini_prompt_tool_new() {
        let prompt = GeminiPromptTool::new("Tool output");
        assert_eq!(prompt.text, "Tool output");
        assert_eq!(prompt.role, Some(GeminiRole::Tool));
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
        assert_eq!(GeminiRole::from_str("user"), Some(GeminiRole::User));
        assert_eq!(GeminiRole::from_str("tool"), Some(GeminiRole::Tool));
        assert_eq!(GeminiRole::from_str("SYSTEM"), Some(GeminiRole::System));
        assert_eq!(GeminiRole::from_str("USER"), Some(GeminiRole::User));
        assert_eq!(GeminiRole::from_str("TOOL"), Some(GeminiRole::Tool));
        assert_eq!(GeminiRole::from_str("unknown"), None);
    }

    #[test]
    fn test_gemini_prompt_conversion_to_request() {
        // Test GeminiPrompt with no role
        let prompt = GeminiPrompt::new("Basic prompt");
        let request = GeminiRequest::from_prompt(&prompt);
        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.contents[0].role, None);

        if let GeminiPart::Text(part) = &request.contents[0].parts[0] {
            assert_eq!(part.text, "Basic prompt");
        } else {
            panic!("Expected GeminiPart::Text");
        }

        // Test GeminiPrompt with System role
        let system_prompt = GeminiPromptSystem::new("System instructions");
        let request = GeminiRequest::from_prompt(&system_prompt);
        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.contents[0].role, Some("system".to_string()));

        // Test GeminiPrompt with User role
        let user_prompt = GeminiPromptUser::new("User question");
        let request = GeminiRequest::from_prompt(&user_prompt);
        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.contents[0].role, Some("user".to_string()));
    }
}

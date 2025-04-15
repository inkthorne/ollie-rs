use crate::gemini::GeminiRole;
use crate::{GeminiPart, GeminiPartCode, GeminiPartText};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ===
// STRUCT: GeminiContent
// ===

/// Represents content for Gemini API requests.
///
/// This struct holds the parts that make up a message to be sent to the Gemini API,
/// with an optional role field to identify the speaker (e.g., "user" or "model").
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    /// The role of the message sender (e.g., "user" or "model").
    /// When None, the role is determined by the API based on context.
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,

    /// The content parts that make up the message.
    /// Can include text, code, or other media types supported by the Gemini API.
    pub parts: Vec<GeminiPart>,
}

// ===
// PUBLIC: GeminiContent
// ===

impl GeminiContent {
    /// Creates a new GeminiContent instance with an empty parts vector.
    ///
    /// # Returns
    /// A new `GeminiContent` with no role set and an empty parts collection.
    pub fn new() -> Self {
        GeminiContent {
            role: None,
            parts: Vec::new(),
        }
    }

    /// Converts the GeminiContent instance to a JSON value.
    ///
    /// # Returns
    /// A `JsonValue` representing this content, or a default JSON value if serialization fails.
    pub fn to_json(&self) -> JsonValue {
        serde_json::to_value(self).unwrap_or_default()
    }

    /// Adds a code snippet to the content's parts.
    ///
    /// # Parameters
    /// * `language` - The programming language of the code snippet
    /// * `code` - The actual code content as a string
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    pub fn add_code(&mut self, language: &str, code: &str) -> &mut Self {
        let part = GeminiPart::Code(GeminiPartCode::new(language, code));

        self.add_part(part)
    }

    /// Adds a part to the content's parts vector.
    ///
    /// # Parameters
    /// * `part` - The `GeminiPart` to add to this content
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    pub fn add_part(&mut self, part: GeminiPart) -> &mut Self {
        self.parts.push(part);
        self
    }

    /// Gets the role assigned to this content.
    ///
    /// # Returns
    /// An `Option<GeminiRole>` containing the role if one is set and valid, or `None` otherwise.
    pub fn role(&self) -> Option<GeminiRole> {
        self.role.as_ref().and_then(|r| GeminiRole::from_str(r))
    }

    /// Sets the role for this content.
    ///
    /// # Parameters
    /// * `role` - The GeminiRole to assign (e.g., GeminiRole::User)
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    pub fn set_role(&mut self, role: GeminiRole) -> &mut Self {
        self.role = Some(role.as_str().to_string());
        self
    }

    /// Adds a text part to the content's parts.
    ///
    /// # Parameters
    /// * `text` - The text content to add
    ///
    /// # Returns
    /// A mutable reference to self for method chaining
    pub fn add_text(&mut self, text: &str) -> &mut Self {
        let part = GeminiPart::Text(GeminiPartText {
            text: text.to_string(),
        });

        self.add_part(part)
    }
}

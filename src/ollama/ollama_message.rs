use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize, Clone)]
pub struct OllamaMessageBuilder {
    #[serde(skip_serializing_if = "String::is_empty")]
    role: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    content: String,
}

impl OllamaMessageBuilder {
    pub fn new() -> Self {
        OllamaMessageBuilder {
            role: String::new(),
            content: String::new(),
        }
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = role.to_string();
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn build(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }
}

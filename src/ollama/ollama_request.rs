use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize, Clone)]
pub struct OllamaRequestBuilder {
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    messages: Vec<JsonValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<JsonValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

impl OllamaRequestBuilder {
    pub fn new() -> Self {
        Self {
            model: String::new(),
            messages: Vec::new(),
            options: None,
            stream: None,
        }
    }

    pub fn build(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }

    pub fn message(mut self, message: JsonValue) -> Self {
        self.messages.push(message);
        self
    }

    pub fn messages(mut self, messages: Vec<JsonValue>) -> Self {
        self.messages = messages;
        self
    }

    pub fn model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub fn options(mut self, options: JsonValue) -> Self {
        self.options = Some(options);
        self
    }
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }
}

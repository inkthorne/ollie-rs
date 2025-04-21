use crate::OllamaMessage2;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaResponse2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    done: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    done_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    eval_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    eval_duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    load_duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<OllamaMessage2>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_eval_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prompt_eval_duration: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_duration: Option<u64>,
}

impl OllamaResponse2 {
    pub fn from_json(json: serde_json::Value) -> Result<Self, serde_json::Error> {
        let response = serde_json::from_value(json)?;
        Ok(response)
    }

    pub fn to_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn print_stats(&self) {
        let model = self.model().unwrap_or("unknown model");
        let eval_tokens = self.eval_count.unwrap_or(0);

        let eval_in_milliseconds = self.eval_duration.unwrap_or(0) / 1_000_000;
        let eval_in_seconds = eval_in_milliseconds as f64 / 1_000.0;

        let token_rate = if eval_in_seconds > 0.0 {
            eval_tokens as f64 / eval_in_seconds as f64
        } else {
            0.0
        };

        let stats = json!({
            "model": model,
            "context_used": self.tokens_used(),
            "eval_time": eval_in_seconds,
            "eval_tokens": eval_tokens,
            "eval_rate": token_rate,
        });

        println!(
            "\n\n-> stats: {}",
            serde_json::to_string_pretty(&stats).unwrap()
        );
    }

    /// Returns the generated text from the model response.
    ///
    /// This method first checks for content in the message field, and if not found,
    /// falls back to the response field. Returns `None` if neither is available.
    pub fn text(&self) -> Option<&str> {
        // Look for the text in the message content first.
        if let Some(message) = self.message() {
            if message.content().is_some() {
                return message.content();
            }
        }

        // If not found, look for the text in the response field.
        if self.response().is_some() {
            return self.response();
        }

        // If neither is found, return None.
        None
    }

    pub fn tokens_used(&self) -> u32 {
        self.eval_count.unwrap_or(0) + self.prompt_eval_count.unwrap_or(0)
    }
}

// ===
// PROPERTIES: OllamaResponse2
// ===

impl OllamaResponse2 {
    /// Returns the creation time of the response.
    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }

    pub fn done(&self) -> Option<&bool> {
        self.done.as_ref()
    }

    pub fn done_reason(&self) -> Option<&str> {
        self.done_reason.as_deref()
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn eval_count(&self) -> Option<&u32> {
        self.eval_count.as_ref()
    }

    pub fn eval_duration(&self) -> Option<&u64> {
        self.eval_duration.as_ref()
    }

    pub fn load_duration(&self) -> Option<&u64> {
        self.load_duration.as_ref()
    }

    pub fn message(&self) -> Option<&OllamaMessage2> {
        self.message.as_ref()
    }

    pub fn set_message(&mut self, message: OllamaMessage2) {
        self.message = Some(message);
    }

    /// Returns the model name used for the response.
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    pub fn prompt_eval_count(&self) -> Option<&u32> {
        self.prompt_eval_count.as_ref()
    }

    pub fn prompt_eval_duration(&self) -> Option<&u64> {
        self.prompt_eval_duration.as_ref()
    }

    pub fn response(&self) -> Option<&str> {
        self.response.as_deref()
    }

    pub fn set_response(&mut self, response: &str) {
        self.response = Some(response.to_string());
    }

    pub fn total_duration(&self) -> Option<&u64> {
        self.total_duration.as_ref()
    }
}

// ===
// TRAIT: Display for OllamaResponse2
// ===

impl fmt::Display for OllamaResponse2 {
    /// Formats the OllamaResponse2 for display using pretty-printed JSON.
    ///
    /// # Arguments
    /// * `f` - The formatter to write the output to
    ///
    /// # Returns
    /// * Result indicating whether the formatting operation succeeded
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_value(self).unwrap();
        let pretty = serde_json::to_string_pretty(&json).unwrap_or_default();
        write!(f, "{}", pretty)
    }
}

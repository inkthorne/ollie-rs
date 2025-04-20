use crate::OllamaMessage2;
use serde::{Deserialize, Serialize};

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
        let tokens = self.eval_count.unwrap_or(0);

        let eval_in_milliseconds = self.eval_duration.unwrap_or(0) / 1_000_000;
        let eval_in_seconds = eval_in_milliseconds as f64 / 1_000.0;

        let token_rate = if eval_in_seconds > 0.0 {
            tokens as f64 / eval_in_seconds as f64
        } else {
            0.0
        };

        println!(
            "\n\n-> model: {model}\n-> tokens: {tokens}\n-> eval time: {eval_in_seconds:.1}s\n-> token rate: {token_rate:.1}/sec"
        );
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

    pub fn total_duration(&self) -> Option<&u64> {
        self.total_duration.as_ref()
    }
}

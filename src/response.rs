use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub done_reason: String,
    pub context: Vec<u32>,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
    // pub prompt_eval_count_duration: u64,
    pub eval_count: u32,
    // pub eval_count_duration: u64,
}

//============================================================================
// OllamaResponse
//============================================================================
pub struct OllamaResponse {
    response: serde_json::Value,
}

impl OllamaResponse {
    pub fn from_slice(slice: &[u8]) -> Result<Self, serde_json::Error> {
        let response = serde_json::from_slice(slice)?;
        Ok(Self { response })
    }

    pub fn as_json(&self) -> &serde_json::Value {
        &self.response
    }

    pub fn done(&self) -> Option<bool> {
        self.response.get("done")?.as_bool()
    }

    pub fn response(&self) -> Option<&str> {
        self.response.get("response")?.as_str()
    }

    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.response).unwrap_or_default()
    }
}

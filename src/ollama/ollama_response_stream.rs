use serde_json::Value as JsonValue;

pub struct OllamaResponseStream {
    http_response: reqwest::Response,
    llm_response: JsonValue,
    accumulated_content: String,
}

impl OllamaResponseStream {
    pub fn new(http_response: reqwest::Response) -> Self {
        Self {
            http_response,
            llm_response: JsonValue::default(),
            accumulated_content: String::new(),
        }
    }

    pub async fn read(&mut self) -> Option<JsonValue> {
        let http_chunk = self.http_response.chunk().await.ok()?;

        if let Some(bytes) = http_chunk {
            let chunk_str = String::from_utf8_lossy(&bytes);
            let chunk: JsonValue = serde_json::from_str(&chunk_str).ok()?;

            if let Some(content) = chunk["message"]["content"].as_str() {
                self.accumulated_content.push_str(content);
            }

            self.llm_response = chunk.clone();
            self.llm_response["message"]["content"] = self.accumulated_content.clone().into();
            return Some(chunk);
        }

        None
    }

    pub fn response(&self) -> JsonValue {
        self.llm_response.clone()
    }
}

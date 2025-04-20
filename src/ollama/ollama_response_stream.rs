use serde_json::Value as JsonValue;
use std::error::Error;

pub struct OllamaResponseStream {
    http_response: reqwest::Response,
    llm_response: JsonValue,
    llm_responses: Vec<JsonValue>,
    save_responses: bool,
}

impl OllamaResponseStream {
    pub fn new(http_response: reqwest::Response) -> Self {
        Self {
            http_response,
            llm_response: JsonValue::default(),
            llm_responses: Vec::new(),
            save_responses: false,
        }
    }

    pub fn save_responses(&mut self, flag: bool) {
        self.save_responses = flag;
    }

    pub async fn read(&mut self) -> Option<JsonValue> {
        let http_chunk = self.http_response.chunk().await.ok()?;

        if let Some(bytes) = http_chunk {
            let chunk_str = String::from_utf8_lossy(&bytes);
            let chunk: JsonValue = serde_json::from_str(&chunk_str).ok()?;

            if self.save_responses {
                self.llm_responses.push(chunk.clone());
            } else {
                self.llm_response = chunk.clone();
            }

            return Some(chunk);
        }

        None
    }

    pub async fn read2<F>(&mut self, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(&JsonValue),
    {
        while let Some(chunk_bytes) = self.http_response.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk_bytes);
            let chunk_json: JsonValue = serde_json::from_str(&chunk_str)?;

            if self.save_responses {
                callback(&chunk_json);
                self.llm_responses.push(chunk_json);
            } else {
                callback(&chunk_json);
                self.llm_response = chunk_json;
            }
        }

        Ok(())
    }

    pub fn response(&self) -> JsonValue {
        // If save_responses is true, we need to accumulate the content and response fields.
        if self.save_responses {
            let mut accumulated_content = String::new();
            let mut accumulated_response = String::new();

            for response in &self.llm_responses {
                if let Some(c) = response["message"]["content"].as_str() {
                    accumulated_content.push_str(c);
                }

                if let Some(r) = response["response"].as_str() {
                    accumulated_response.push_str(r);
                }
            }

            let mut response = self.llm_responses.last().unwrap().clone();

            if !accumulated_content.is_empty() {
                response["message"]["content"] = accumulated_content.into();
            }

            if !accumulated_response.is_empty() {
                response["response"] = accumulated_response.into();
            }

            return response;
        }

        // If save_responses is false, return the last response.
        self.llm_response.clone()
    }
}

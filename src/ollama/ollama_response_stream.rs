use serde_json::Value as JsonValue;

pub struct OllamaResponseStream {
    http_response: reqwest::Response,
}

impl OllamaResponseStream {
    pub fn new(http_response: reqwest::Response) -> Self {
        Self { http_response }
    }

    pub async fn read(&mut self) -> Option<JsonValue> {
        let http_chunk = self.http_response.chunk().await.ok()?;

        if let Some(bytes) = http_chunk {
            let chunk_str = String::from_utf8_lossy(&bytes);
            let chunk: JsonValue = serde_json::from_str(&chunk_str).ok()?;

            return Some(chunk);
        }

        None
    }
}

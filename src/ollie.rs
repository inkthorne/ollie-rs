use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
pub struct GenerateResponse {
    pub response: String,
}

pub struct Ollie {
    server_addr: SocketAddr,
}

impl Default for Ollie {
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::from_str("127.0.0.1:11434").unwrap(),
        }
    }
}

impl Ollie {
    pub fn new(server_addr: SocketAddr) -> Self {
        Self { server_addr }
    }

    pub fn server_addr(&self) -> &SocketAddr {
        &self.server_addr
    }

    pub async fn generate(&self, model: String, prompt: String) -> Result<String, reqwest::Error> {
        let url = format!("http://{}/api/generate", self.server_addr);
        let client = reqwest::Client::new();
        let request = GenerateRequest { model, prompt };
        let response = client.post(&url).json(&request).send().await?;
        let text = response.text().await?;
        println!("Text: {:?}", text);

        return Ok(text);
        /*
        let json_response = response.json::<GenerateResponse>().await?;
        Ok(json_response)
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate() {
        let ollie = Ollie::default();
        let result = ollie
            .generate(
                "gemma3:4b".to_string(),
                "What is the capital of France?".to_string(),
            )
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_empty());
    }
}

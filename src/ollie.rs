use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

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

    pub async fn generate(
        &self,
        model: String,
        prompt: String,
    ) -> Result<GenerateResponse, reqwest::Error> {
        let url = format!("http://{}/api/generate", self.server_addr);
        let client = reqwest::Client::new();
        let request = GenerateRequest {
            model,
            prompt,
            stream: false,
        };
        let response = client.post(&url).json(&request).send().await?;

        // Parse the response into a strongly-typed GenerateResponse
        let generate_response = response.json::<GenerateResponse>().await?;
        Ok(generate_response)
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

        if let Err(ref e) = result {
            println!("Error in generate: {:?}", e);
        }
        assert!(result.is_ok());
        let response = result.unwrap();
        println!("{:?}", response);
    }
}

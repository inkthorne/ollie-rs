use serde::Deserialize;
use std::net::SocketAddr;
use std::str::FromStr;

//============================================================================
// OllamaGenerateRequest
//============================================================================
/// Represents a request to the Ollama API
///
/// This struct is used to build requests for the Ollama API using a fluent interface.
pub struct OllamaGenerateRequest {
    value: serde_json::Value,
}

impl OllamaGenerateRequest {
    /// Creates a new empty Ollama request
    ///
    /// ## Returns
    ///
    /// A new `OllamaRequest` instance with default values
    pub fn new() -> Self {
        Self {
            value: serde_json::Value::default(),
        }
    }

    /// Sets the model to use for the request
    ///
    /// ## Arguments
    ///
    /// * `model` - The name of the model to use (e.g., "gemma3:4b", "llama3")
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn model(&mut self, model: String) -> &mut Self {
        self.value["model"] = serde_json::Value::String(model);
        self
    }

    /// Sets the prompt text for the request
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The prompt text to send to the model
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn prompt(&mut self, prompt: String) -> &mut Self {
        self.value["prompt"] = serde_json::Value::String(prompt);
        self
    }

    /// Sets whether the response should be streamed
    ///
    /// ## Arguments
    ///
    /// * `prompt` - Boolean indicating if the response should be streamed
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn stream(&mut self, prompt: bool) -> &mut Self {
        self.value["stream"] = serde_json::Value::Bool(prompt);
        self
    }

    /// Sets the requested output format
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The format to request (e.g., "json")
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn format(&mut self, prompt: String) -> &mut Self {
        self.value["format"] = serde_json::Value::String(prompt);
        self
    }

    /// Returns the underlying JSON value of the request
    ///
    /// ## Returns
    ///
    /// A reference to the internal JSON value
    pub fn as_json(&self) -> &serde_json::Value {
        &self.value
    }
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponseChunk {
    pub model: String,
    // pub created_at: String,
    pub response: String,
    pub done: bool,
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

//============================================================================
// Ollama
//============================================================================
pub struct Ollama {
    server_addr: SocketAddr,
    http_client: reqwest::Client,
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::from_str("127.0.0.1:11434").unwrap(),
            http_client: reqwest::Client::new(),
        }
    }
}

impl Ollama {
    /// Creates a new Ollama client with the specified server address
    ///
    /// ## Arguments
    ///
    /// * `server_addr` - Socket address (IP and port) where the Ollama server is running
    ///
    /// ## Returns
    ///
    /// A new `Ollama` instance connected to the specified server address
    pub fn new(server_addr: SocketAddr) -> Self {
        Self {
            server_addr,
            http_client: reqwest::Client::new(),
        }
    }

    /// Returns the server address this client is configured to connect to
    ///
    /// ## Returns
    ///
    /// A reference to the socket address where the Ollama server is running
    pub fn server_addr(&self) -> &SocketAddr {
        &self.server_addr
    }

    /// Sends a generation request to the Ollama server and returns the response
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The request containing model, prompt text, and other generation parameters
    ///
    /// ## Returns
    ///
    /// * `Ok(String)` - The generated text response from the model
    /// * `Err(reqwest::Error)` - Any network or server errors that occurred
    ///
    /// ## Note
    ///
    /// This function handles streaming responses by collecting chunks until completion.
    /// For streamed responses, it parses each chunk as a JSON object and concatenates
    /// the response text together.
    pub async fn generate(&self, prompt: &OllamaGenerateRequest) -> Result<String, reqwest::Error> {
        let url = format!("http://{}/api/generate", self.server_addr);
        let mut response = self
            .http_client
            .post(&url)
            .json(prompt.as_json())
            .send()
            .await?;

        let mut output = String::new();
        let mut chunk_bytes = Vec::<u8>::new();

        while let Some(http_chunk) = response.chunk().await? {
            chunk_bytes.extend_from_slice(&http_chunk);

            // Use the OllamaResponseChunk struct for proper deserialization
            match serde_json::from_slice::<OllamaResponseChunk>(&chunk_bytes) {
                Ok(chunk) => {
                    if chunk.done {
                        break;
                    }
                    output.push_str(&chunk.response);
                    chunk_bytes.clear();
                }
                Err(_) => {
                    // If we can't parse a complete JSON object yet, continue collecting chunks
                    println!("chunk_bytes: {}", String::from_utf8_lossy(&chunk_bytes));
                    continue;
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_json_request() {
        let ollama = Ollama::default();
        let mut prompt = OllamaGenerateRequest::new();
        prompt
            .model("gemma3:4b".to_string())
            .prompt("What is the capital of France? respond in json".to_string())
            .stream(true)
            .format("json".to_string());

        let result = ollama.generate(&prompt).await;

        if let Err(ref e) = result {
            println!("Error in request: {:?}", e);
        }
        assert!(result.is_ok());

        // Parse the string response into a JSON Value
        let response_str = result.unwrap();
        println!("response string: {}", response_str);
        let json_value: Value = serde_json::from_str(&response_str).expect("Failed to parse JSON");
        println!("response json: {:?}", json_value);
    }
}

use crate::request::OllamaRequest;
use serde::Deserialize;
use std::net::SocketAddr;
use std::str::FromStr;

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
    /// * `response_handler` - Callback function that processes each JSON response chunk
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
    pub async fn generate<F>(
        &self,
        prompt: &OllamaRequest,
        mut response_handler: F,
    ) -> Result<String, reqwest::Error>
    where
        F: FnMut(serde_json::Value),
    {
        let url = format!("http://{}/api/generate", self.server_addr);
        let mut response = self
            .http_client
            .post(&url)
            .json(prompt.as_json())
            .send()
            .await?;

        let mut accumulated_response = String::new();

        while let Some(http_chunk) = response.chunk().await? {
            match serde_json::from_slice::<serde_json::Value>(&http_chunk) {
                Ok(json_chunk) => {
                    if let Some(response_chunk) =
                        json_chunk.get("response").and_then(|r| r.as_str())
                    {
                        accumulated_response.push_str(response_chunk);
                    }
                    response_handler(json_chunk);
                }
                Err(_) => {
                    continue;
                }
            }
        }

        Ok(accumulated_response)
    }
}

//============================================================================
// TESTS
//============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{OllamaFunction, OllamaFunctionParameters, OllamaTools};
    use serde_json::Value;

    #[tokio::test]
    async fn test_generate_request1() {
        let ollama = Ollama::default();
        let mut request = OllamaRequest::new();
        request
            .model("gemma3:1b")
            .prompt("What is the capital of France? respond in json")
            .stream(false)
            .format("json");

        let result = ollama
            .generate(&request, |response| {
                let pretty = serde_json::to_string_pretty(&response).unwrap();
                println!("response: {}", pretty);
            })
            .await;

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

    #[tokio::test]
    async fn test_generate_request_with_tools() {
        // Create a new Ollama client with default settings
        let ollama = Ollama::default();

        // Create the tools collection
        let mut tools = OllamaTools::new();

        // Create a search function for retrieving information
        let mut temperature_function = OllamaFunction::new(
            "get_temperature",
            "Gets the current temperature for a location.",
        );

        // Add parameters to the function
        let mut params = OllamaFunctionParameters::new();
        params.parameter(
            "location",
            "string",
            "the location to get the temperature for",
            true,
        );
        temperature_function.parameters(params);
        tools.add_function(temperature_function);

        // Create the request with a prompt that would trigger tool usage
        let mut request = OllamaRequest::new();
        request
            .model("gemma3:4b")
            .prompt("What is the current temperature in Seattle? Please use your tools & respond in JSON.")
            .stream(true)
            .tools(&tools);

        // Generate a response using the request with tools
        let result = ollama
            .generate(&request, |response| {
                let pretty = serde_json::to_string_pretty(&response).unwrap();
                println!("response: {}", pretty);
            })
            .await;

        // Verify the request was successful
        if let Err(ref e) = result {
            println!("Error in request with tools: {:?}", e);
        }
        assert!(result.is_ok());

        // Check the response (actual verification would depend on the Ollama instance)
        let response_str = result.unwrap();
        println!("Tool response: {}", response_str);

        // Note: In a real test with a mocked Ollama server, we would verify that:
        // 1. The tool was called with the appropriate parameters
        // 2. The response contained the expected tool call information
        // 3. The model handled the tool response correctly

        // For now, just printing the response for manual inspection
    }
}

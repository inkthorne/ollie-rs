use crate::request::OllamaRequest;
use crate::response::OllamaResponse;
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
    ) -> Result<(), reqwest::Error>
    where
        F: FnMut(OllamaResponse),
    {
        let url = format!("http://{}/api/generate", self.server_addr);
        let mut response = self
            .http_client
            .post(&url)
            .json(prompt.as_json())
            .send()
            .await?;

        while let Some(http_chunk) = response.chunk().await? {
            match OllamaResponse::from_slice(&http_chunk) {
                Ok(ollama_response) => {
                    response_handler(ollama_response);
                }
                Err(_) => {
                    continue;
                }
            }
        }

        Ok(())
    }
}

//============================================================================
// TESTS
//============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{OllamaFunction, OllamaFunctionParameters, OllamaTools};

    /// Tests basic text generation functionality with the Ollama API
    ///
    /// This test:
    /// 1. Creates a default Ollama client
    /// 2. Sends a simple prompt asking about France's capital
    /// 3. Configures the response to be in JSON format
    /// 4. Verifies that the request completes successfully
    #[tokio::test]
    async fn test_generate_request1() {
        let ollama = Ollama::default();
        let mut request = OllamaRequest::new();
        request
            .model("gemma3:1b")
            .prompt("What is the capital of France? respond in json")
            .stream(false)
            .format("json");

        let mut accumulated_response = String::new();
        let result = ollama
            .generate(&request, |response| {
                response
                    .response()
                    .map(|r| accumulated_response.push_str(r));
                println!("response: {}", response.to_string_pretty());
            })
            .await;

        if let Err(ref e) = result {
            println!("Error in request: {:?}", e);
        }
        assert!(result.is_ok());

        // Print the accumulated response for manual verification
        println!("accumulated_response: {}", accumulated_response);
    }

    /// Tests the Ollama API's function calling capabilities using custom tools
    ///
    /// This test:
    /// 1. Sets up an Ollama client
    /// 2. Creates a custom tool for getting temperature data
    /// 3. Sends a prompt that should trigger tool usage
    /// 4. Verifies the request processes successfully with tool integration
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
        let mut accumulated_response = String::new();
        let result = ollama
            .generate(&request, |response| {
                response
                    .response()
                    .map(|r| accumulated_response.push_str(r));
                println!("response: {}", response.to_string_pretty());
            })
            .await;

        // Verify the request was successful
        if let Err(ref e) = result {
            println!("Error in request with tools: {:?}", e);
        }
        assert!(result.is_ok());

        // Print the accumulated response for manual inspection
        println!("Tool response: {}", accumulated_response);

        // Note: In a real test with a mocked Ollama server, we would verify that:
        // 1. The tool was called with the appropriate parameters
        // 2. The response contained the expected tool call information
        // 3. The model handled the tool response correctly
    }
}

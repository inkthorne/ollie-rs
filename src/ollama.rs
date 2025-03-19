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
        response_handler: F,
    ) -> Result<(), reqwest::Error>
    where
        F: FnMut(OllamaResponse),
    {
        let url = format!("http://{}/api/generate", self.server_addr);
        self.send_request(&url, prompt, response_handler).await
    }

    /// Sends a chat request to the Ollama server and returns the response
    ///
    /// ## Arguments
    ///
    /// * `prompt` - The request containing model, messages, and other chat parameters
    /// * `response_handler` - Callback function that processes each JSON response chunk
    ///
    /// ## Returns
    ///
    /// * `Ok(())` - If the request completed successfully
    /// * `Err(reqwest::Error)` - Any network or server errors that occurred
    ///
    /// ## Note
    ///
    /// This function is similar to `generate` but uses the chat API endpoint.
    /// It handles streaming responses by collecting chunks until completion.
    pub async fn chat<F>(
        &self,
        prompt: &OllamaRequest,
        response_handler: F,
    ) -> Result<(), reqwest::Error>
    where
        F: FnMut(OllamaResponse),
    {
        let url = format!("http://{}/api/chat", self.server_addr);
        self.send_request(&url, prompt, response_handler).await
    }

    /// Sends a request to a specific Ollama API endpoint and processes the response
    ///
    /// ## Arguments
    ///
    /// * `url` - The complete URL for the API endpoint
    /// * `prompt` - The request containing model, prompt text, and other parameters
    /// * `response_handler` - Callback function that processes each JSON response chunk
    ///
    /// ## Returns
    ///
    /// * `Ok(())` - If the request completed successfully
    /// * `Err(reqwest::Error)` - Any network or server errors that occurred
    ///
    /// ## Note
    ///
    /// This function is similar to `generate` but allows specifying the API endpoint URL directly.
    /// It handles streaming responses by collecting chunks until completion.
    pub async fn send_request<F>(
        &self,
        url: &str,
        prompt: &OllamaRequest,
        mut response_handler: F,
    ) -> Result<(), reqwest::Error>
    where
        F: FnMut(OllamaResponse),
    {
        let mut response = self
            .http_client
            .post(url)
            .json(prompt.as_json())
            .send()
            .await?;

        while let Some(http_chunk) = response.chunk().await? {
            match OllamaResponse::try_from(&http_chunk) {
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
    use crate::{
        message::OllamaMessage,
        tool::{OllamaFunction, OllamaFunctionParameters, OllamaTools},
    };

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
            .set_model("gemma3:1b")
            .set_prompt("What is the capital of France? respond in json")
            .set_stream(false)
            .set_format("json");

        let mut accumulated_response = String::new();
        let result = ollama
            .generate(&request, |response| {
                response
                    .response()
                    .map(|r| accumulated_response.push_str(r));
                println!("response: {}", response.as_string_pretty());
            })
            .await;

        if let Err(ref e) = result {
            println!("Error in request: {:?}", e);
        }
        assert!(result.is_ok());

        // Print the accumulated response for manual verification
        println!("accumulated_response: {}", accumulated_response);
    }

    /// Tests basic chat functionality with the Ollama API
    ///
    /// This test:
    /// 1. Creates a default Ollama client
    /// 2. Sends a simple prompt asking about France's capital
    /// 3. Configures the response to be in JSON format
    /// 4. Verifies that the request completes successfully
    #[tokio::test]
    async fn test_chat_request1() {
        let ollama = Ollama::default();
        let mut message = OllamaMessage::new();
        message
            .set_role("user")
            .set_content("can you explain briefly, why is the sky blue?");

        let mut request = OllamaRequest::new();
        request
            .set_model("gemma3:1b")
            .set_stream(true)
            .push_message(&message);

        let mut accumulated_content = String::new();
        let result = ollama
            .chat(&request, |response| {
                response.content().map(|c| accumulated_content.push_str(c));
                // println!("response: {}", response.to_string_pretty());
            })
            .await;

        if let Err(ref e) = result {
            println!("Error in request: {:?}", e);
        }
        assert!(result.is_ok());

        // Print the accumulated response for manual verification
        println!("accumulated_content: {}", accumulated_content);
    }

    /// Tests the Ollama API's function calling capabilities using custom tools
    ///
    /// This test:
    /// 1. Sets up an Ollama client
    /// 2. Creates a custom tool for getting temperature data
    /// 3. Sends a prompt that should trigger tool usage
    /// 4. Verifies the request processes successfully with tool integration
    #[tokio::test]
    async fn test_chat_request_with_tools() {
        // Create a new Ollama client with default settings
        let ollama = Ollama::default();

        // Create the tools collection
        let mut tools = OllamaTools::new();

        // Create a search function for retrieving information
        let mut temperature_function = OllamaFunction::new(
            "get_current_weather",
            "Gets the current weather for a location.",
        );

        // Add parameters to the function
        let mut params = OllamaFunctionParameters::new();
        params.push_parameter(
            "location",
            "string",
            "the location to get the temperature for",
            true,
        );
        temperature_function.set_parameters(params);
        tools.push_function(temperature_function);

        let mut message = OllamaMessage::new();
        message
            .set_role("user")
            .set_content("What is the current weather in Paris?");

        // Create the request with a prompt that would trigger tool usage
        let mut request = OllamaRequest::new();
        request
            .set_model("llama3.2")
            .set_stream(false)
            .set_tools(&tools)
            .push_message(&message);
        println!("---\nrequest: {}", request.as_string_pretty());

        let mut forwarded_tool_call = OllamaMessage::new();

        // Generate a response using the request with tools
        let mut accumulated_response = String::new();
        let result = ollama
            .chat(&request, |response| {
                response
                    .response()
                    .map(|r| accumulated_response.push_str(r));

                println!("---\nresponse: {}", response.as_string_pretty());

                response.message().map(|message| {
                    println!("---\nmessage: {}", message.as_string_pretty());
                    message.tool_calls().map(|tool_calls| {
                        for i in 0..tool_calls.len() {
                            let tool_call = tool_calls.tool_call(i).unwrap();
                            println!("---\ntool_call: {}", tool_call.as_string_pretty());
                        }
                        forwarded_tool_call = message.into();
                    });
                });
            })
            .await;

        // Verify the request was successful
        if let Err(ref e) = result {
            println!("Error in request with tools: {:?}", e);
        }
        assert!(result.is_ok());

        // Print the accumulated response for manual inspection
        println!("Tool response: {}", accumulated_response);

        let mut tool_response = OllamaMessage::new();
        tool_response
            .set_role("tool")
            .set_content("{ \"temperature\": \"40°C\"")
            .set_name("get_current_weather");

        request.push_message(&forwarded_tool_call);
        request.push_message(&tool_response);
        println!("---\n2nd request: {}", request.as_string_pretty());

        // Generate a 2nd response using context from the tool
        let mut accumulated_response = String::new();
        let result = ollama
            .chat(&request, |response| {
                response
                    .response()
                    .map(|r| accumulated_response.push_str(r));

                println!("---\nresponse: {}", response.as_string_pretty());
            })
            .await;

        assert!(result.is_ok());

        // Print the accumulated response for manual inspection
        println!("2nd response: {}", accumulated_response);

        // Note: In a real test with a mocked Ollama server, we would verify that:
        // 1. The tool was called with the appropriate parameters
        // 2. The response contained the expected tool call information
        // 3. The model handled the tool response correctly
    }
}

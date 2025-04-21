use crate::OllamaResponse2;
use crate::{OllamaRequest, OllamaRequest2, OllamaResponse};
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;

// ===
// STRUCT: Ollama
// ===

/// Client for interacting with the Ollama API.
///
/// This struct provides methods for sending requests to an Ollama server
/// and processing the responses. It supports both the 'generate' and 'chat'
/// endpoints, as well as handling streaming responses.
pub struct Ollama {
    /// The network address (IP and port) where the Ollama server is running
    server_addr: SocketAddr,
    /// HTTP client used for making requests to the Ollama server
    http_client: reqwest::Client,
}

impl Ollama {
    /// Creates a new Ollama client with the specified server address
    ///
    /// ## Arguments
    ///
    /// * `server_addr_str` - String address (e.g., "127.0.0.1:11434") where the Ollama server is running
    ///
    /// ## Returns
    ///
    /// A new `Ollama` instance connected to the specified server address
    ///
    /// ## Panics
    ///
    /// This function will panic if the provided string cannot be parsed as a valid socket address
    pub fn new(server_addr_str: &str) -> Self {
        Self {
            server_addr: SocketAddr::from_str(server_addr_str).unwrap(),
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
    /// * `Ok(Option<OllamaResponse>)` - The final response if successful, or None if no response was received
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
    ) -> Result<Option<OllamaResponse>, reqwest::Error>
    where
        F: FnMut(&OllamaResponse),
    {
        let url = format!("http://{}/api/generate", self.server_addr);
        self.send_request(&url, prompt, response_handler).await
    }

    /// Sends a generation request to the Ollama server and processes the response with a callback
    ///
    /// ## Arguments
    ///
    /// * `request` - An `OllamaRequest2` object containing the model, prompt, and other generation parameters
    /// * `callback` - A function that will be called with each response chunk as it arrives
    ///
    /// ## Returns
    ///
    /// * `Ok(OllamaResponse2)` - The final response if successful
    /// * `Err(Box<dyn Error>)` - Any error that occurred during the request or processing
    pub async fn generate3<F>(
        &self,
        request: &OllamaRequest2,
        callback: F,
    ) -> Result<OllamaResponse2, Box<dyn Error>>
    where
        F: FnMut(&OllamaResponse2),
    {
        let url = format!("http://{}/api/generate", self.server_addr);
        self.request3(&url, request, callback).await
    }

    /// Sends a chat request using an OllamaRequest2 object and processes response chunks with a callback.
    ///
    /// This method sends a chat request to the Ollama server and processes each response chunk
    /// through the provided callback function. Unlike `chat2`, this method handles the chunked
    /// responses internally and returns the final response.
    ///
    /// ## Arguments
    ///
    /// * `request` - An `OllamaRequest2` object containing the model, messages, and other chat parameters.
    /// * `callback` - A function that will be called with each response chunk as it arrives.
    ///
    /// ## Returns
    ///
    /// * `Ok(OllamaResponse2)` - The final response if successful.
    /// * `Err(Box<dyn Error>)` - Any error that occurred during the request or processing.
    pub async fn chat3<F>(
        &self,
        request: &OllamaRequest2,
        callback: F,
    ) -> Result<OllamaResponse2, Box<dyn Error>>
    where
        F: FnMut(&OllamaResponse2),
    {
        let url = format!("http://{}/api/chat", self.server_addr);
        self.request3(&url, request, callback).await
    }

    /// Sends an HTTP POST request with a JSON payload and processes the response with a callback.
    ///
    /// This is a helper function used by `generate3` and `chat3`.
    ///
    /// ## Arguments
    ///
    /// * `url` - The target URL for the POST request.
    /// * `request` - An `OllamaRequest2` object containing the request parameters.
    /// * `callback` - A function that will be called with each response chunk as it arrives.
    ///
    /// ## Returns
    ///
    /// * `Ok(OllamaResponse2)` - The final response if successful.
    /// * `Err(Box<dyn Error>)` - Any error that occurred during the request or processing.
    pub async fn request3<F>(
        &self,
        url: &str,
        request: &OllamaRequest2,
        mut callback: F,
    ) -> Result<OllamaResponse2, Box<dyn Error>>
    where
        F: FnMut(&OllamaResponse2),
    {
        // Send a POST request to the Ollama server with the JSON payload.
        let mut http_response = self.http_client.post(url).json(request).send().await?;
        let mut response = None;
        let mut accumulated_text = String::new();

        while let Some(chunk_bytes) = http_response.chunk().await? {
            // Deserialize the chunk into a OllamaRequest object.
            let chunk_string = String::from_utf8_lossy(&chunk_bytes);
            let chunk_json = serde_json::from_str(&chunk_string)?;
            let chunk_response = OllamaResponse2::from_json(chunk_json)?;

            // Accumulate the content text (if streaming).
            if let Some(text) = chunk_response.text() {
                accumulated_text.push_str(text);
            }

            // Forward the response to the callback.
            callback(&chunk_response);
            response = Some(chunk_response);
        }

        let streaming = request.stream().unwrap_or(true);

        // If streaming, set the accumulated text in the final response.
        if streaming {
            if let Some(r) = &mut response {
                // If the request contains messages, set the accumulated text as the final response.
                if let Some(message) = r.message() {
                    let message = message.clone().set_content(&accumulated_text);
                    r.set_message(message);
                } else {
                    // Otherwise, set the accumulated text as the final response.
                    r.set_response(&accumulated_text);
                }
            }
        }

        Ok(response.unwrap())
    }

    /// Sends a chat request to the Ollama server and returns the response
    ///
    /// ## Arguments
    ///
    /// * `request` - The request containing model, messages, and other chat parameters
    /// * `response_handler` - Callback function that processes each JSON response chunk
    ///
    /// ## Returns
    ///
    /// * `Ok(Option<OllamaResponse>)` - The final response if successful, or None if no response was received
    /// * `Err(reqwest::Error)` - Any network or server errors that occurred
    ///
    /// ## Note
    ///
    /// This function is similar to `generate` but uses the chat API endpoint.
    /// It handles streaming responses by collecting chunks until completion.
    pub async fn chat<F>(
        &self,
        request: &OllamaRequest,
        response_handler: F,
    ) -> Result<Option<OllamaResponse>, reqwest::Error>
    where
        F: FnMut(&OllamaResponse),
    {
        let url = format!("http://{}/api/chat", self.server_addr);
        self.send_request(&url, request, response_handler).await
    }

    /// Sends a request to a specific Ollama API endpoint and processes the response
    ///
    /// ## Arguments
    ///
    /// * `url` - The complete URL for the API endpoint
    /// * `request` - The request containing model, prompt text, and other parameters
    /// * `response_handler` - Callback function that processes each JSON response chunk
    ///
    /// ## Returns
    ///
    /// * `Ok(Option<OllamaResponse>)` - The final response if successful, or None if no response was received
    /// * `Err(reqwest::Error)` - Any network or server errors that occurred
    ///
    /// ## Note
    ///
    /// This function is similar to `generate` but allows specifying the API endpoint URL directly.
    /// It handles streaming responses by collecting chunks until completion.
    pub async fn send_request<F>(
        &self,
        url: &str,
        request: &OllamaRequest,
        mut response_handler: F,
    ) -> Result<Option<OllamaResponse>, reqwest::Error>
    where
        F: FnMut(&OllamaResponse),
    {
        let mut response = self
            .http_client
            .post(url)
            .json(request.as_json())
            .send()
            .await?;

        let mut last_response: Option<OllamaResponse> = Option::None;
        let mut accumulated_content = String::new();

        while let Some(http_chunk) = response.chunk().await? {
            match OllamaResponse::try_from(&http_chunk) {
                Ok(ollama_response) => {
                    // Forward the response to the streaming handler.
                    response_handler(&ollama_response);

                    // Accumulate the content (if streaming).
                    if request.stream() {
                        ollama_response.response().map(|r| {
                            accumulated_content.push_str(r);
                        });

                        ollama_response.content().map(|c| {
                            accumulated_content.push_str(c);
                        });
                    }

                    last_response = Some(ollama_response);
                }
                Err(_) => {
                    continue;
                }
            }
        }

        // Put the accumulated content into the last response (if streaming).
        if request.stream() {
            if let Some(ref mut r) = last_response {
                if r.message().is_some() {
                    r.set_content(&accumulated_content);
                } else {
                    r.set_response(&accumulated_content);
                }
            }
        }

        Ok(last_response)
    }
}

// ===
// TRAIT: Default for Ollama
// ===

impl Default for Ollama {
    /// Creates a new Ollama client with default configuration (localhost:11434)
    ///
    /// ## Returns
    ///
    /// A new `Ollama` instance connected to 127.0.0.1:11434
    fn default() -> Self {
        Self {
            server_addr: SocketAddr::from_str("127.0.0.1:11434").unwrap(),
            http_client: reqwest::Client::new(),
        }
    }
}

// ===
// TESTS: Ollama
// ===
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        OllamaFunction, OllamaFunctionParameters, OllamaMessage, OllamaMessage2, OllamaTools,
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
            .set_stream(true)
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
        let message = OllamaMessage2::new()
            .set_role("user")
            .set_content("can you explain briefly, why is the sky blue?")
            .to_json();

        let request = OllamaRequest2::new()
            .set_model("gemma3:1b")
            .add_message(message);

        let mut accumulated_content = String::new();
        let result = ollama
            .chat3(&request, |response| {
                // Append response content to accumulated content
                if let Some(text) = response.text() {
                    accumulated_content.push_str(text);
                }
                // println!("response: {}", response.as_string_pretty());
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
            .add_message(&message);
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

        request.add_message(&forwarded_tool_call);
        request.add_message(&tool_response);
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

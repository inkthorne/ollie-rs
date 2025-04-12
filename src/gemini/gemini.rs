use crate::{GeminiRequest, GeminiResponse, GeminiResponseStream};
use reqwest::Response as HttpResponse;
use serde_json::Value as JsonValue;
use std::error::Error;

const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

// ===
// STRUCT: Gemini
// ===

pub struct Gemini {
    /// The name of the model to use for content generation.
    model: String,

    /// The API key used for authentication with the Gemini API.
    api_key: String,

    /// The base URL for the Gemini API.
    base_url: String,

    /// HTTP client used for making requests to the Gemini server.
    https_client: reqwest::Client,
}

// ===
// PUBLIC IMPL: Gemini
// ===

impl Gemini {
    /// Creates a new instance of the Gemini struct with default settings.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use for Gemini API requests.
    ///
    /// # Returns
    ///
    /// * `Gemini` - An instance of the Gemini struct.
    pub fn new(model: &str, api_key: &str) -> Self {
        Gemini {
            model: model.to_string(),
            api_key: api_key.to_string(),
            base_url: GEMINI_BASE_URL.to_string(),
            https_client: reqwest::Client::new(),
        }
    }

    /// Sets a custom base URL for the Gemini API.
    ///
    /// This can be useful for testing or when using a proxy server.
    ///
    /// # Arguments
    ///
    /// * `url` - The new base URL to use for Gemini API requests.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to this instance for method chaining.
    pub fn set_base_url(&mut self, url: &str) -> &mut Self {
        self.base_url = url.to_string();
        self
    }

    /// Returns the current base URL used for Gemini API requests.
    ///
    /// # Returns
    ///
    /// * `&str` - The current base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Sends a content generation request to the Gemini API and returns the response.
    ///
    /// # Arguments
    ///
    /// * `request` - A GeminiRequest containing the request content for the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<GeminiResponse, Box<dyn Error>>` - A GeminiResponse containing the API response if successful,
    ///   or an error if the request failed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The HTTP request fails
    /// * The API response cannot be parsed as text
    /// * The response text cannot be parsed as JSON
    pub async fn generate(
        &self,
        request: &GeminiRequest,
    ) -> Result<GeminiResponse, Box<dyn Error>> {
        // Construct the request URL.
        let url = format!(
            "{}/{}:generateContent?key={}",
            self.base_url, self.model, self.api_key
        );

        // Send the HTTP request.
        let response = self
            .https_client
            .post(&url)
            .json(request.as_json())
            .send()
            .await;

        // If there's an HTTP error, return it.
        if let Err(err) = response {
            return Err(err.without_url().into());
        }

        let response = response.unwrap();
        let text = response.text().await;

        if let Err(err) = text {
            return Err(err.without_url().into());
        }

        let text = text.unwrap();
        // Parse the response text into a JSON value
        let json_value: JsonValue = serde_json::from_str(&text)?;
        let gemini_response = GeminiResponse::new(json_value);
        Ok(gemini_response)
    }

    /// Sends a streaming content generation request to the Gemini API and returns a stream wrapper.
    ///
    /// This method allows for streaming responses from the Gemini API, which is useful for
    /// real-time processing of generated content. The response is returned as a GeminiResponseStream
    /// that can be processed for server-sent events (SSE).
    ///
    /// # Arguments
    ///
    /// * `request` - A GeminiRequest containing the request content for the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<GeminiResponseStream, Box<dyn Error>>` - A stream wrapper if successful, or an error
    ///   if the request failed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The HTTP request fails
    /// * The API returns a non-success status code
    pub async fn generate_stream(
        &self,
        request: &GeminiRequest,
    ) -> Result<GeminiResponseStream, Box<dyn Error>> {
        // Construct the request URL.
        let url = format!(
            "{}/{}:streamGenerateContent?alt=sse&key={}",
            self.base_url, self.model, self.api_key
        );

        // Send the HTTP request.
        let response = self
            .https_client
            .post(&url)
            .json(request.as_json())
            .send()
            .await;

        // Return the HTTP response or the error.
        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    let error = format!("{}", response.status());
                    return Err(error.into());
                }

                return Ok(GeminiResponseStream::new(response));
            }
            Err(err) => {
                return Err(err.without_url().into());
            }
        }
    }

    /// Retrieves a list of available models from the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<JsonValue, Box<dyn Error>>` - The API response containing model information as a
    ///   JSON value if successful, or an error if the request failed.
    pub async fn list_models(&self) -> Result<JsonValue, Box<dyn Error>> {
        let url = format!("{}?key={}", self.base_url, self.api_key);
        let response = self.https_client.get(&url).send().await;

        if let Err(err) = response {
            return Err(err.without_url().into());
        }

        let response = response.unwrap();
        let text = response.text().await;

        if let Err(err) = text {
            return Err(err.without_url().into());
        }

        let text = text.unwrap();
        // Parse the response text into a JSON value
        let json_value: JsonValue = serde_json::from_str(&text)?;
        Ok(json_value)
    }

    /// Processes and extracts JSON data from a streaming HTTP response from the Gemini API.
    ///
    /// This method reads a chunk from the HTTP response stream, parses it according to the
    /// server-sent events (SSE) format used by Gemini, and converts it to a JSON value.
    /// It performs the following steps:
    /// 1. Reads a chunk from the HTTP response
    /// 2. Converts the bytes to a UTF-8 string
    /// 3. Extracts the data after the "data:" prefix in the SSE format
    /// 4. Parses the extracted data as JSON
    ///
    /// Note that this method silently handles errors by returning None in several cases:
    /// - When chunk() returns an error
    /// - When the chunk is None (end of stream)
    /// - When UTF-8 conversion fails
    /// - When "data:" prefix cannot be found
    /// - When JSON parsing fails
    ///
    /// # Arguments
    ///
    /// * `response` - A mutable reference to an HTTP response that supports streaming.
    ///
    /// # Returns
    ///
    /// * `Option<JsonValue>` - A JSON value if a chunk was successfully read and parsed,
    ///   or None if the stream has ended or an error occurred during parsing.
    pub async fn read_stream(response: &mut HttpResponse) -> Option<JsonValue> {
        let bytes = response.chunk().await.ok()?;

        if bytes.is_none() {
            return None;
        }

        let bytes = bytes.unwrap();
        let string = String::from_utf8(bytes.to_vec()).ok()?;
        let slice = string.split_once("data:")?.1;
        let value: JsonValue = serde_json::from_str(&slice).ok()?;

        return Some(value);
    }
}

// ===
// TESTS: Gemini
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GeminiRequest;
    use std::env;

    fn api_key() -> String {
        env::var("GEMINI_API_KEY").expect("-> Error: environment variable GEMINI_API_KEY")
    }

    /// Tests the `set_base_url` method and its accessor to ensure they properly
    /// modify and return the base URL for the Gemini API.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance
    /// 2. Verifies the default base URL is set correctly
    /// 3. Sets a custom base URL
    /// 4. Confirms the custom URL was properly set
    /// 5. Tests method chaining by setting another URL and making assertions
    #[test]
    fn test_gemini_set_base_url() {
        let mut gemini = Gemini::new("gemini-1.0-pro", "dummy_api_key");

        // Verify default base URL
        assert_eq!(gemini.base_url(), GEMINI_BASE_URL);

        // Test setting a custom URL
        let custom_url = "https://custom-api.example.com/v1/models";
        gemini.set_base_url(custom_url);
        assert_eq!(gemini.base_url(), custom_url);

        // Test method chaining
        let another_url = "https://another-api.example.org/v2/models";
        let result = gemini.set_base_url(another_url);
        assert_eq!(result.base_url(), another_url);
    }

    /// Tests the `list_models` method of the Gemini struct to ensure it successfully
    /// retrieves the list of available models from the Gemini API.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance
    /// 2. Makes a real API call to fetch available models
    /// 3. Verifies that the response is successful
    /// 4. Prints the response content showing the available models
    #[tokio::test]
    async fn test_gemini_list_models() {
        let gemini = Gemini::new("gemini-1.0-pro", &api_key());
        let response = gemini.list_models().await;

        if let Err(err) = &response {
            assert!(response.is_ok(), "{err}");
        }

        let response = response.unwrap();
        let pretty_json = serde_json::to_string_pretty(&response).unwrap();
        println!("Models: {pretty_json}");
    }

    /// Tests the `generate_stream` method of the Gemini struct to ensure it successfully sends
    /// a streaming content generation request to the Gemini API and processes the response.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance
    /// 2. Constructs a test prompt asking to explain AI
    /// 3. Makes a real API call to Gemini to request a streaming response
    /// 4. Verifies that the response is successful
    /// 5. Processes and prints each chunk of the streaming response
    ///
    /// Note: Requires the GEMINI_API_KEY environment variable to be set.
    /// Note: The test uses "gemma-3-27b-it" model by default, with "gemini-2.0-flash" as an alternative option.
    #[tokio::test]
    async fn test_gemini_generate_stream() {
        // Model selection
        let model = "gemma-3-27b-it"; // Alternative: "gemini-2.0-flash"
        let gemini = Gemini::new(model, &api_key());

        let request = GeminiRequest::text("Explain how AI works in a few sentences.");
        let stream = gemini.generate_stream(&request).await;

        if let Err(err) = &stream {
            assert!(stream.is_ok(), "{err}");
        }

        let mut stream = stream.unwrap();

        while let Some(response) = stream.response().await {
            println!("{}\n", response.to_string_pretty());
        }
    }

    /// Tests the `generate` method of the Gemini struct to ensure it successfully sends
    /// a content generation request to the Gemini API and receives a valid response.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance
    /// 2. Constructs a test prompt asking to explain AI
    /// 3. Makes a real API call to Gemini (requires valid API key)
    /// 4. Verifies that the response is successful
    /// 5. Prints the response content as a pretty-formatted JSON
    ///
    /// Note: Requires the GEMINI_API_KEY environment variable to be set.
    /// Note: The test uses "gemma-3-27b-it" model by default, with "gemini-2.0-flash" as an alternative option.
    #[tokio::test]
    async fn test_gemini_generate_nostream() {
        // Model selection
        let model = "gemma-3-27b-it"; // Alternative: "gemini-2.0-flash"
        let gemini = Gemini::new(model, &api_key());

        let request = GeminiRequest::text("Explain how AI works in a few sentences.");
        let gemini_response = gemini.generate(&request).await;

        match &gemini_response {
            Ok(gemini_response) => {
                let pretty_json = gemini_response.to_string_pretty();
                println!("Response: {pretty_json}");
            }
            Err(err) => {
                assert!(gemini_response.is_ok(), "{err}");
            }
        }
    }
}

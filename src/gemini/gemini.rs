use reqwest::Response as HttpResponse;
use serde_json::Value as JsonValue;
use std::env;
use std::error::Error;

static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

// ===
// STRUCT: Gemini
// ===

pub struct Gemini {
    api_key: String,

    /// HTTP client used for making requests to the Gemini server.
    https_client: reqwest::Client,
}

// ===
// PUBLIC IMPL: Gemini
// ===

impl Gemini {
    /// Creates a new instance of the Gemini struct with default settings.
    ///
    /// # Returns
    ///
    /// * `Gemini` - An instance of the Gemini struct initialized with the default model "gemma-3-27b-it".
    pub fn new(api_key: &str) -> Self {
        Gemini {
            api_key: api_key.to_string(),
            https_client: reqwest::Client::new(),
        }
    }

    /// Sends a content generation request to the Gemini API and returns the response.
    ///
    /// # Arguments
    ///
    /// * `model` - The model name to use for this specific request.
    /// * `content` - A JSON value containing the request content for the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<String, String>` - The API response as a String if successful, or an error message if the request failed.
    pub async fn generate(
        &self,
        model: &str,
        content: &serde_json::Value,
    ) -> Result<String, String> {
        let api_key = Gemini::api_key();

        if api_key.is_none() {
            return Err("Error: GEMINI_API_KEY environment variable is not set.".to_string());
        }

        let api_key = api_key.unwrap();
        let url = format!("{GEMINI_BASE_URL}/{model}:generateContent?key={api_key}");
        let response = self.https_client.post(&url).json(&content).send().await;

        if let Err(err) = response {
            let error = Gemini::reqwest_error_string(err);
            return Err(error);
        }

        let response = response.unwrap();
        let text = response.text().await;

        if let Err(err) = text {
            let error = Gemini::reqwest_error_string(err);
            return Err(error);
        }

        let text = text.unwrap();
        Ok(text)
    }

    /// Sends a streaming content generation request to the Gemini API and returns the raw HTTP response.
    ///
    /// This method allows for streaming responses from the Gemini API, which is useful for
    /// real-time processing of generated content. The response is returned as a raw HTTP response
    /// that can be processed for server-sent events (SSE).
    ///
    /// # Arguments
    ///
    /// * `content` - A JSON value containing the request content for the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<HttpResponse, Box<dyn Error>>` - The raw HTTP response if successful, or an error
    ///   if the request failed.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The API key is not set in the environment variables
    /// * The HTTP request fails
    /// * The API returns a non-success status code
    pub async fn generate_stream(
        &self,
        model: &str,
        content: &JsonValue,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        // Construct the request URL.
        let url = format!(
            "{GEMINI_BASE_URL}/{}:streamGenerateContent?alt=sse&key={}",
            model, self.api_key
        );

        // Send the HTTP request.
        let response = self.https_client.post(&url).json(&content).send().await;

        // Return the HTTP response or the error.
        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    let error = format!("{}", response.status());
                    return Err(error.into());
                }

                return Ok(response);
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
    /// * `Result<String, String>` - The API response containing model information as a
    ///   String if successful, or an error message if the request failed.
    pub async fn list_models(&self) -> Result<String, String> {
        let api_key = Gemini::api_key().unwrap();
        let url = format!("{GEMINI_BASE_URL}?key={api_key}");
        let response = self.https_client.get(&url).send().await;

        if let Err(err) = response {
            let error = Gemini::reqwest_error_string(err);
            return Err(error);
        }

        let response = response.unwrap();
        let text = response.text().await;

        if let Err(err) = text {
            let error = Gemini::reqwest_error_string(err);
            return Err(error);
        }

        let text = text.unwrap();
        Ok(text)
    }

    /// Processes and extracts JSON data from a streaming HTTP response from the Gemini API.
    ///
    /// This method reads a chunk from the HTTP response stream, parses it according to the
    /// server-sent events (SSE) format used by Gemini, and converts it to a JSON value.
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
// PRIVATE IMPL: Gemini
// ===

impl Gemini {
    /// Retrieves the Google API key for Gemini from the environment variables.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The API key as a String if the environment variable is set,
    ///   otherwise None.
    fn api_key() -> Option<String> {
        env::var("GEMINI_API_KEY").ok()
    }

    /// Formats a reqwest Error into a String for error reporting.
    ///
    /// This method removes sensitive URL information from the error message.
    ///
    /// # Arguments
    ///
    /// * `err` - The reqwest Error to format.
    ///
    /// # Returns
    ///
    /// * `String` - A formatted error message string.
    fn reqwest_error_string(err: reqwest::Error) -> String {
        format!("{}", err.without_url())
    }
}

// ===
// TESTS: Gemini
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Tests the `generate` method of the Gemini struct to ensure it successfully sends
    /// a content generation request to the Gemini API and receives a valid response.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance with the default model
    /// 2. Constructs a test prompt asking to explain AI
    /// 3. Makes a real API call to Gemini (requires valid API key)
    /// 4. Verifies that the response is successful
    /// 5. Prints the response content
    ///
    /// Note: Requires the GEMINI_API_KEY environment variable to be set.
    #[tokio::test]
    async fn test_gemini_generate_nostream() {
        let api_key = env::var("GEMINI_API_KEY").ok().unwrap();
        let model = "gemma-3-27b-it";
        // let model = "gemini-2.0-flash";
        let gemini = Gemini::new(&api_key);

        // Test content
        let content = json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": "Explain how AI works in a few sentences."
                        }
                    ]
                }
            ]
        });

        let response = gemini.generate(model, &content).await;

        if let Err(err) = &response {
            assert!(response.is_ok(), "{err}");
        }

        let response = response.unwrap();
        print!("{response}");
    }

    /// Tests the `list_models` method of the Gemini struct to ensure it successfully
    /// retrieves the list of available models from the Gemini API.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance
    /// 2. Makes a real API call to fetch available models
    /// 3. Verifies that the response is successful
    /// 4. Prints the response content showing the available models
    ///
    /// Note: Requires the GEMINI_API_KEY environment variable to be set.
    #[tokio::test]
    async fn test_gemini_list_models() {
        let api_key = env::var("GEMINI_API_KEY").ok().unwrap();
        let gemini = Gemini::new(&api_key);

        let response = gemini.list_models().await;

        if let Err(err) = &response {
            assert!(response.is_ok(), "{err}");
        }

        let response = response.unwrap();
        print!("\nContent: {response}");
    }

    /// Tests the `generate_stream` method of the Gemini struct to ensure it successfully sends
    /// a streaming content generation request to the Gemini API and processes the response.
    ///
    /// This test:
    /// 1. Creates a new Gemini instance with the default model
    /// 2. Constructs a test prompt asking to explain AI
    /// 3. Makes a real API call to Gemini to request a streaming response
    /// 4. Verifies that the response is successful
    /// 5. Processes and prints each chunk of the streaming response
    ///
    /// Note: Requires the GEMINI_API_KEY environment variable to be set.
    #[tokio::test]
    async fn test_gemini_generate_stream() {
        let api_key = env::var("GEMINI_API_KEY").ok().unwrap();
        let model = "gemma-3-27b-it";
        // let model = "gemini-2.0-flash";
        let content = json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": "Explain how AI works in a few sentences."
                        }
                    ]
                }
            ]
        });

        let gemini = Gemini::new(&api_key);
        let response = gemini.generate_stream(model, &content).await;

        if let Err(err) = &response {
            assert!(response.is_ok(), "{err}");
        }

        let mut response = response.unwrap();

        while let Some(json_chunk) = Gemini::read_stream(&mut response).await {
            let chunk_string = serde_json::to_string_pretty(&json_chunk).unwrap();
            println!("{}\n", chunk_string);
        }
    }
}

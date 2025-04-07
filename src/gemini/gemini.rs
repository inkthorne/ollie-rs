use std::env;

static GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

// ===
// Gemini Struct
// ===

pub struct Gemini {
    model: String,

    /// HTTP client used for making requests to the Gemini server.
    https_client: reqwest::Client,
}

// ---
// Gemini Public
// ---

impl Gemini {
    /// Creates a new instance of the Gemini struct with default settings.
    ///
    /// # Returns
    ///
    /// * `Gemini` - An instance of the Gemini struct initialized with the default model "gemma-3-27b-it".
    pub fn new() -> Self {
        Gemini {
            model: "gemma-3-27b-it".to_string(),
            https_client: reqwest::Client::new(),
        }
    }

    /// Returns a reference to the currently set model.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice referencing the current model.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Sets a new model to be used for requests.
    ///
    /// # Arguments
    ///
    /// * `model` - The model name to use for requests. Can be any type that can be converted into a String.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - A mutable reference to this instance for method chaining.
    pub fn set_model(&mut self, model: impl Into<String>) -> &mut Self {
        self.model = model.into();
        self
    }

    /// Sends a content generation request to the Gemini API and returns the response.
    ///
    /// # Arguments
    ///
    /// * `content` - A JSON value containing the request content for the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<String, String>` - The API response as a String if successful, or an error message if the request failed.
    pub async fn post(&self, content: &serde_json::Value) -> Result<String, String> {
        let api_key = Gemini::get_api_key();
        let api_key = api_key.unwrap();

        let model = &self.model;
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

    /// Retrieves a list of available models from the Gemini API.
    ///
    /// # Returns
    ///
    /// * `Result<String, String>` - The API response containing model information as a
    ///   String if successful, or an error message if the request failed.
    pub async fn list_models(&self) -> Result<String, String> {
        let api_key = Gemini::get_api_key().unwrap();
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
}

// ---
// Gemini Private
// ---

impl Gemini {
    /// Retrieves the Google API key for Gemini from the environment variables.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The API key as a String if the environment variable is set,
    ///   otherwise None.
    fn get_api_key() -> Option<String> {
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

// ---
// Gemini Tests
// ---

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Tests the `post` method of the Gemini struct to ensure it successfully sends
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
    async fn test_post_success() {
        // let model = "gemini-2.0-flash";
        // let model = "gemma-3-27b-it";
        let gemini = Gemini::new();

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

        let response = gemini.post(&content).await;

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
    async fn test_list_models() {
        let gemini = Gemini::new();

        let response = gemini.list_models().await;

        if let Err(err) = &response {
            assert!(response.is_ok(), "{err}");
        }

        let response = response.unwrap();
        print!("\nContent: {response}");
    }
}

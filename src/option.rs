use serde::Serialize;
use serde_json::Value;

/// Configuration options for Ollama API requests.
///
/// This struct provides a builder pattern for setting various parameters
/// that control how the Ollama API handles requests.
#[derive(Serialize)]
pub struct OllamaOptions {
    /// The context size (in tokens) to consider for the model's response.
    ///
    /// Larger values allow the model to consider more context when generating responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    num_ctx: Option<u32>,

    /// Controls the randomness of the model's output.
    ///
    /// Values closer to 0 make the output more deterministic and focused,
    /// while values closer to 1 make output more creative and diverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

impl OllamaOptions {
    /// Creates a new `OllamaOptions` instance with default settings (all options set to None).
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::option::OllamaOptions;
    ///
    /// let options = OllamaOptions::new();
    /// ```
    pub fn new() -> Self {
        OllamaOptions {
            num_ctx: None,
            temperature: None,
        }
    }

    /// Sets the context window size for the model.
    ///
    /// # Arguments
    ///
    /// * `num_ctx` - The number of tokens to consider for context
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn set_num_ctx(&mut self, num_ctx: u32) -> &mut Self {
        self.num_ctx = Some(num_ctx);
        self
    }

    /// Sets the temperature parameter for the model.
    ///
    /// # Arguments
    ///
    /// * `temperature` - A value typically between 0.0 and 1.0 that controls randomness
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn set_temperature(&mut self, temperature: f32) -> &mut Self {
        self.temperature = Some(temperature);
        self
    }

    /// Converts the options to a JSON value for serialization.
    ///
    /// # Returns
    ///
    /// A serde_json Value containing the options, or Null if serialization fails
    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

//============================================================================
// TESTS
//============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_json_empty() {
        let options = OllamaOptions::new();
        let json = options.to_json();
        println!("{:?}", json);

        assert!(json.is_object());
        assert!(json.get("num_ctx").is_none());
        assert!(json.get("temperature").is_none());
    }

    #[test]
    fn test_option_to_json_with_values() {
        let mut options = OllamaOptions::new();
        options.set_num_ctx(4096).set_temperature(0.75);
        let json = options.to_json();
        println!("{:?}", json);

        assert!(json.is_object());
        assert_eq!(json["num_ctx"], 4096);
        assert_eq!(json["temperature"], 0.75);
    }
}

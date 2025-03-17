use crate::message::{OllamaMessage, OllamaMessages};
use crate::tool::OllamaTools;

//============================================================================
// OllamaRequest
//============================================================================
/// Represents a request to the Ollama API
///
/// This struct is used to build requests for the Ollama API using a fluent interface.
pub struct OllamaRequest {
    request: serde_json::Value,
}

impl OllamaRequest {
    /// Creates a new empty Ollama request
    ///
    /// ## Returns
    ///
    /// A new `OllamaRequest` instance with default values
    pub fn new() -> Self {
        Self {
            request: serde_json::Value::default(),
        }
    }

    pub fn messages(&mut self, messages: OllamaMessages) -> &mut Self {
        self.request["messages"] = messages.as_json().clone();
        self
    }

    pub fn push_message(&mut self, message: &OllamaMessage) -> &mut Self {
        if !self.request.as_object().unwrap().contains_key("messages") {
            self.request["messages"] = serde_json::Value::Array(vec![]);
        }

        if let Some(messages) = self.request["messages"].as_array_mut() {
            messages.push(message.as_json().clone());
        }

        self
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
    pub fn model(&mut self, model: &str) -> &mut Self {
        self.request["model"] = model.into();
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
    pub fn prompt(&mut self, prompt: &str) -> &mut Self {
        self.request["prompt"] = prompt.into();
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
    pub fn stream(&mut self, stream: bool) -> &mut Self {
        self.request["stream"] = stream.into();
        self
    }

    /// Sets the requested output format
    ///
    /// ## Arguments
    ///
    /// * `format` - The format to request (e.g., "json")
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn format(&mut self, format: &str) -> &mut Self {
        self.request["format"] = format.into();
        self
    }

    /// Returns whether streaming is enabled for this request
    ///
    /// ## Returns
    ///
    /// `true` if streaming is enabled, `false` otherwise
    pub fn is_streamed(&self) -> bool {
        self.request
            .get("stream")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
    }

    /// Adds tools to the request
    ///
    /// ## Arguments
    ///
    /// * `tools` - The OllamaTools instance containing defined functions
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn tools(&mut self, tools: &OllamaTools) -> &mut Self {
        self.request["tools"] = tools.as_json().clone();
        self
    }

    /// Returns the underlying JSON value of the request
    ///
    /// ## Returns
    ///
    /// A reference to the internal JSON value
    pub fn as_json(&self) -> &serde_json::Value {
        &self.request
    }

    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.request).unwrap_or_default()
    }
}

//============================================================================
// TESTS
//============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{OllamaFunction, OllamaFunctionParameters};

    #[test]
    fn test_is_streamed() {
        let mut request = OllamaRequest::new();
        assert!(
            request.is_streamed(),
            "New request should be streamed by default"
        );

        request.stream(false);
        assert!(
            !request.is_streamed(),
            "Request should not be streamed after disabling"
        );

        request.stream(true);
        assert!(
            request.is_streamed(),
            "Request should be streamed after enabling"
        );
    }

    /// Tests creating OllamaRequest objects and verifying their JSON representation
    ///
    /// This test verifies that:
    /// - Two different requests can be created with different settings
    /// - The fluent interface correctly builds the underlying JSON structure
    /// - Different prompts and streaming settings are properly represented
    #[test]
    fn test_json_request() {
        // Create first request - with streaming enabled
        let mut request1 = OllamaRequest::new();
        request1
            .model("llama3:8b")
            .prompt("What are the top 5 machine learning frameworks?")
            .stream(true);

        // Create second request - without streaming
        let mut request2 = OllamaRequest::new();
        request2
            .model("gemma3:4b")
            .prompt("Explain quantum computing in simple terms")
            .stream(false)
            .format("json");

        // Print the JSON values for debugging
        println!(
            "---\nRequest 1 JSON: {}",
            serde_json::to_string_pretty(request1.as_json()).unwrap()
        );
        println!(
            "---\nRequest 2 JSON: {}",
            serde_json::to_string_pretty(request2.as_json()).unwrap()
        );

        // Verify first request has correct values
        let json1 = request1.as_json();
        assert_eq!(json1["model"], "llama3:8b");
        assert_eq!(
            json1["prompt"],
            "What are the top 5 machine learning frameworks?"
        );
        assert_eq!(json1["stream"], true);
        assert!(!json1.as_object().unwrap().contains_key("format"));

        // Verify second request has correct values
        let json2 = request2.as_json();
        assert_eq!(json2["model"], "gemma3:4b");
        assert_eq!(json2["prompt"], "Explain quantum computing in simple terms");
        assert_eq!(json2["stream"], false);
        assert_eq!(json2["format"], "json");
    }

    /// Tests creating an OllamaRequest with tools
    ///
    /// This test verifies that:
    /// - An OllamaRequest can be created with tools
    /// - The tools are properly added to the JSON structure
    /// - The final request contains both model configuration and tools
    #[test]
    fn test_json_request_with_tools() {
        // Create a new tools instance
        let mut tools = OllamaTools::new();

        // Create a function for getting weather
        let mut weather_function =
            OllamaFunction::new("get_weather", "Get current weather data for a location");

        // Add parameters to the function
        let mut params = OllamaFunctionParameters::new();
        params
            .parameter(
                "location",
                "string",
                "City and state (e.g., Seattle, WA)",
                true,
            )
            .parameter(
                "unit",
                "string",
                "Temperature unit (celsius or fahrenheit)",
                false,
            );

        weather_function.parameters(params);

        // Add the function to tools
        tools.add_function(weather_function);

        // Create the request with the tools
        let mut request = OllamaRequest::new();
        request
            .model("llama3:8b")
            .prompt("What's the weather like in Seattle?")
            .stream(false)
            .tools(&tools);

        // Print the JSON for debugging
        println!(
            "---\nRequest with tools JSON: {}",
            serde_json::to_string_pretty(request.as_json()).unwrap()
        );

        // Verify the request has the expected structure
        let json = request.as_json();
        assert_eq!(json["model"], "llama3:8b");
        assert_eq!(json["prompt"], "What's the weather like in Seattle?");
        assert_eq!(json["stream"], false);

        // Verify tools structure
        assert!(json.as_object().unwrap().contains_key("tools"));

        // Verify the tools array contains our function
        if let Some(tools_array) = json["tools"].as_array() {
            assert_eq!(tools_array.len(), 1);

            // Check the type field
            assert_eq!(tools_array[0]["type"], "function");

            // Access the nested function object
            if let Some(function) = tools_array[0]["function"].as_object() {
                assert_eq!(function["name"], "get_weather");
                assert_eq!(
                    function["description"],
                    "Get current weather data for a location"
                );

                // Verify parameters structure
                if let Some(params) = function["parameters"].as_object() {
                    assert!(params.contains_key("properties"));
                    assert!(params.contains_key("required"));

                    // Verify location parameter
                    let properties = &function["parameters"]["properties"];
                    assert_eq!(properties["location"]["type"], "string");
                    assert_eq!(
                        properties["location"]["description"],
                        "City and state (e.g., Seattle, WA)"
                    );

                    // Verify required parameters
                    let required = &function["parameters"]["required"];
                    assert!(required.as_array().unwrap().contains(&"location".into()));
                } else {
                    panic!("Expected parameters to be an object");
                }
            } else {
                panic!("Expected function to be an object");
            }
        } else {
            panic!("Expected tools to be an array");
        }
    }
}

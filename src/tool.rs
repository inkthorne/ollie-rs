//============================================================================
// OllamaToolCall
//============================================================================
/// ## Example JSON structure
/// ```json
/// {
///   "function": {
///     "arguments": {
///       "location": "Paris"
///     },
///     "name": "get_current_weather"
///   }
/// }
/// ```
pub struct OllamaToolCall {
    value: serde_json::Value,
}

impl OllamaToolCall {
    /// Converts the tool call to a pretty-printed JSON string.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of the tool call.
    /// Returns an empty string if serialization fails.
    pub fn as_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.value).unwrap_or_default()
    }

    /// Extracts the function name from the tool call.
    ///
    /// ## Returns
    ///
    /// An Option containing the function name as a string slice if present,
    /// or None if the function name cannot be found or is not a string.
    pub fn name(&self) -> Option<&str> {
        self.value.get("function")?.get("name")?.as_str()
    }

    /// Extracts the function arguments from the tool call.
    ///
    /// ## Returns
    ///
    /// An Option containing a reference to the JSON value of arguments if present,
    /// or None if the arguments cannot be found.
    pub fn arguments(&self) -> Option<&serde_json::Value> {
        self.value.get("function")?.get("arguments")
    }
}

/// Implements conversion from a JSON value reference to an OllamaToolCall.
impl From<&serde_json::Value> for OllamaToolCall {
    /// Creates an OllamaToolCall from a JSON value reference.
    ///
    /// ## Arguments
    ///
    /// * `value` - A reference to a serde_json::Value to convert
    ///
    /// ## Returns
    ///
    /// A new OllamaToolCall containing a clone of the provided value.
    fn from(value: &serde_json::Value) -> Self {
        Self {
            value: value.clone(),
        }
    }
}

//============================================================================
// OllamaToolCalls
//============================================================================
/// A collection of tool calls for Ollama API communication.
///
/// This struct maintains an array of tool calls that can be sent to or received from Ollama API endpoints.
/// It handles the proper formatting of tool call collections and provides methods
/// for adding and accessing tool calls in the collection.
///
/// ## Example JSON structure
/// ```json
/// [
///   {
///     "function": {
///       "arguments": {
///         "location": "Paris"
///       },
///       "name": "get_current_weather"
///     }
///   },
///   {
///     "function": {
///       "arguments": {
///         "text": "Hello world"
///       },
///       "name": "text_translator"
///     }
///   }
/// ]
/// ```
pub struct OllamaToolCalls {
    array: serde_json::Value,
}

impl OllamaToolCalls {
    /// Creates a new empty OllamaToolCalls collection.
    ///
    /// ## Returns
    ///
    /// A new OllamaToolCalls with an empty array.
    pub fn new() -> Self {
        Self {
            array: serde_json::Value::Array(vec![]),
        }
    }

    /// Gets the underlying JSON representation of the tool call collection.
    ///
    /// ## Returns
    ///
    /// A reference to the underlying JSON value containing the tool call array.
    pub fn as_json(&self) -> &serde_json::Value {
        &self.array
    }

    /// Converts the tool call collection to a pretty-printed JSON string.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string representation of all tool calls.
    /// Returns an empty string if serialization fails.
    pub fn as_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.array).unwrap_or_default()
    }

    /// Returns the number of tool calls in the collection.
    ///
    /// ## Returns
    ///
    /// An integer count of the tool calls in the array.
    pub fn len(&self) -> usize {
        match self.array.as_array() {
            Some(arr) => arr.len(),
            None => 0,
        }
    }

    /// Checks if the collection is empty.
    ///
    /// ## Returns
    ///
    /// A boolean indicating whether the collection has no tool calls.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Retrieves a tool call at the specified index.
    ///
    /// ## Arguments
    ///
    /// * `index` - The zero-based index of the tool call to retrieve
    ///
    /// ## Returns
    ///
    /// An Option containing the OllamaToolCall if the index is valid, or None if out of bounds.
    pub fn tool_call(&self, index: usize) -> Option<OllamaToolCall> {
        match self.array.as_array() {
            Some(arr) => arr.get(index).map(OllamaToolCall::from),
            None => None,
        }
    }

    /// Adds a tool call to the collection.
    ///
    /// ## Arguments
    ///
    /// * `tool_call` - The OllamaToolCall to add to the collection
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn push_tool_call(&mut self, tool_call: OllamaToolCall) -> &mut Self {
        self.array
            .as_array_mut()
            .unwrap()
            .push(tool_call.value.clone());
        self
    }
}

impl From<&serde_json::Value> for OllamaToolCalls {
    fn from(value: &serde_json::Value) -> Self {
        // For array values, keep as-is
        if value.is_array() {
            Self {
                array: value.clone(),
            }
        } else {
            // For non-array values, create a new array with the value as the only element
            Self {
                array: serde_json::Value::Array(vec![value.clone()]),
            }
        }
    }
}

//============================================================================
// OllamaFunctionParameters
//============================================================================
pub struct OllamaFunctionParameters {
    object: serde_json::Value,
}

impl OllamaFunctionParameters {
    pub fn new() -> Self {
        Self {
            object: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }

    /// Returns a pretty-printed JSON string representation of the function parameters.
    ///
    /// ## Returns
    ///
    /// A formatted JSON string of the parameters. Returns an empty string if serialization fails.
    pub fn as_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.object).unwrap_or_default()
    }

    /// Adds a parameter to the function parameters definition.
    ///
    /// ## Arguments
    ///
    /// * `name` - The parameter name as it will appear in the JSON schema
    /// * `json_type` - The JSON schema type (e.g., "string", "number", "boolean")
    /// * `description` - A helpful description of what the parameter is used for
    /// * `required` - Whether this parameter is required (true) or optional (false)
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn push_parameter(
        &mut self,
        name: &str,
        json_type: &str,
        description: &str,
        required: bool,
    ) -> &mut Self {
        let details = serde_json::json!({
            "type": json_type,
            "description": description,
        });

        // Add the new parameter to properties
        self.object["properties"][name] = details;

        // If parameter is required, add it to the required array
        if required {
            self.object["required"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!(name));
        }

        self
    }
}

//============================================================================
// OllamaFunction
//============================================================================
pub struct OllamaFunction {
    object: serde_json::Value,
}

impl OllamaFunction {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            object: serde_json::json!({
                "type": "function",
                "function": {
                    "name": name,
                    "description": description,
                }
            }),
        }
    }
    pub fn set_parameters(&mut self, parameters: OllamaFunctionParameters) -> &mut Self {
        self.object["function"]["parameters"] = parameters.object;
        self
    }
}

//============================================================================
// OllamaTools
//============================================================================
pub struct OllamaTools {
    array: serde_json::Value,
}

impl OllamaTools {
    /// Creates a new empty OllamaTools collection.
    ///
    /// ## Returns
    ///
    /// A new OllamaTools with an empty array.
    pub fn new() -> Self {
        Self {
            array: serde_json::Value::Array(vec![]),
        }
    }

    /// Returns the underlying JSON value of the tools
    ///
    /// ## Returns
    ///
    /// A reference to the internal JSON value
    pub fn as_json(&self) -> &serde_json::Value {
        &self.array
    }

    /// Adds a function to the tools collection.
    ///
    /// ## Arguments
    ///
    /// * `function` - The OllamaFunction to add to the collection
    ///
    /// ## Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn push_function(&mut self, function: OllamaFunction) -> &mut Self {
        self.array.as_array_mut().unwrap().push(function.object);
        self
    }
}

//============================================================================
// TESTS
//============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the OllamaToolCalls collection functionality.
    ///
    /// This test verifies that:
    /// - A new tool calls collection can be created
    /// - Individual OllamaToolCall instances can be added to the collection
    /// - The builder pattern works for chaining operations
    /// - Tool calls can be retrieved by index
    /// - Length and emptiness can be checked
    #[test]
    fn test_tool_calls() {
        // Create a new tool calls collection
        let mut tool_calls = OllamaToolCalls::new();

        // Verify it starts empty
        assert_eq!(tool_calls.len(), 0);
        assert!(tool_calls.is_empty());

        // Create tool calls using JSON values
        let weather_call = OllamaToolCall::from(&serde_json::json!({
            "function": {
                "arguments": {
                    "location": "Tokyo"
                },
                "name": "get_current_weather"
            }
        }));

        let translate_call = OllamaToolCall::from(&serde_json::json!({
            "function": {
                "arguments": {
                    "text": "Hello world",
                    "target_language": "Japanese"
                },
                "name": "translate_text"
            }
        }));

        // Add both tool calls to the collection using builder pattern
        tool_calls
            .push_tool_call(weather_call)
            .push_tool_call(translate_call);

        // Verify the collection is no longer empty
        assert_eq!(tool_calls.len(), 2);
        assert!(!tool_calls.is_empty());

        // Test accessing by valid indices
        let call0 = tool_calls.tool_call(0);
        assert!(call0.is_some());
        assert_eq!(call0.unwrap().name(), Some("get_current_weather"));

        let call1 = tool_calls.tool_call(1);
        assert!(call1.is_some());
        assert_eq!(call1.unwrap().name(), Some("translate_text"));

        // Test accessing by invalid index
        let call_out_of_bounds = tool_calls.tool_call(2);
        assert!(call_out_of_bounds.is_none());

        // Get the JSON string representation and verify it contains our tool calls
        let json_str = tool_calls.as_string_pretty();
        println!("---\ntool_calls: {}", json_str);

        assert!(json_str.contains("Tokyo"));
        assert!(json_str.contains("get_current_weather"));
        assert!(json_str.contains("Hello world"));
        assert!(json_str.contains("translate_text"));
    }

    /// Tests the OllamaFunctionParameters struct functionality.
    ///
    /// This test verifies that:
    /// - Parameters can be added with the parameter() method
    /// - Required and optional parameters are stored correctly
    /// - The JSON structure is properly formed with properties and required arrays
    #[test]
    fn test_json_function_parameters() {
        // Create a new instance of OllamaFunctionParameters
        let mut params = OllamaFunctionParameters::new();

        // Add two string parameters
        params
            .push_parameter("name", "string", "The name of the user", true)
            .push_parameter("email", "string", "The email address of the user", false);

        // Print the value of the structure
        println!(
            "---\nparameters: {}",
            serde_json::to_string_pretty(&params.object).unwrap()
        );

        // Basic assertion to verify the parameters were added
        if let serde_json::Value::Object(properties) = &params.object["properties"] {
            assert!(properties.contains_key("name"));
            assert!(properties.contains_key("email"));
        } else {
            panic!("Expected properties to be an object");
        }

        // Verify that only required parameters are in the required array
        if let serde_json::Value::Array(required) = &params.object["required"] {
            assert_eq!(required.len(), 1);
            assert_eq!(required[0], "name");
            assert!(!required.contains(&serde_json::json!("email")));
        } else {
            panic!("Expected required to be an array");
        }
    }

    /// Tests the OllamaFunction struct functionality.
    ///
    /// This test verifies that:
    /// - A function can be created with name and description
    /// - Parameters can be attached to a function
    /// - The JSON structure correctly contains all function details
    /// - Both required and optional parameters are properly structured
    #[test]
    fn test_json_function() {
        // Create a new function for getting weather
        let mut weather_function = OllamaFunction::new(
            "get_current_weather",
            "Get the current weather in a given location",
        );

        // Create parameters for the function
        let mut params = OllamaFunctionParameters::new();
        params
            .push_parameter(
                "location",
                "string",
                "The city and state, e.g., San Francisco, CA",
                true,
            )
            .push_parameter(
                "format",
                "string",
                "The temperature unit to use: 'celsius' or 'fahrenheit'",
                false,
            );

        // Add parameters to the function
        weather_function.set_parameters(params);

        // Print the value of the structure
        println!(
            "---\n{}",
            serde_json::to_string_pretty(&weather_function.object).unwrap()
        );

        // Verify the function has the expected structure
        if let serde_json::Value::Object(obj) = &weather_function.object {
            assert_eq!(obj["type"], "function");

            // Access the nested function object
            if let serde_json::Value::Object(function) = &obj["function"] {
                assert_eq!(function["name"], "get_current_weather");
                assert_eq!(
                    function["description"],
                    "Get the current weather in a given location"
                );

                // Verify parameters structure
                if let serde_json::Value::Object(params) = &function["parameters"] {
                    // Check properties
                    if let serde_json::Value::Object(properties) = &params["properties"] {
                        assert!(properties.contains_key("location"));
                        assert!(properties.contains_key("format"));

                        // Verify location details
                        assert_eq!(properties["location"]["type"], "string");
                        assert_eq!(
                            properties["location"]["description"],
                            "The city and state, e.g., San Francisco, CA"
                        );

                        // Verify format details
                        assert_eq!(properties["format"]["type"], "string");
                        assert_eq!(
                            properties["format"]["description"],
                            "The temperature unit to use: 'celsius' or 'fahrenheit'"
                        );
                    } else {
                        panic!("Expected properties to be an object");
                    }

                    // Verify required parameters
                    if let serde_json::Value::Array(required) = &params["required"] {
                        assert_eq!(required.len(), 1);
                        assert_eq!(required[0], "location");
                        assert!(!required.contains(&serde_json::json!("format")));
                    } else {
                        panic!("Expected required to be an array");
                    }
                } else {
                    panic!("Expected parameters to be an object");
                }
            } else {
                panic!("Expected function to be an object");
            }
        } else {
            panic!("Expected object to be an object");
        }
    }

    /// Tests the OllamaTools struct functionality.
    ///
    /// This test verifies that:
    /// - Multiple functions can be added to the tools collection
    /// - Each function maintains its structure with parameters
    /// - The JSON structure is properly formed as an array of functions
    #[test]
    fn test_json_tools() {
        // Create a new OllamaTools instance
        let mut tools = OllamaTools::new();

        // Create temperature function
        let mut temp_function = OllamaFunction::new(
            "get_current_temperature",
            "Get the current temperature in a given location",
        );

        // Add parameters to temperature function
        let mut temp_params = OllamaFunctionParameters::new();
        temp_params
            .push_parameter(
                "location",
                "string",
                "The city and state, e.g., San Francisco, CA",
                true,
            )
            .push_parameter(
                "unit",
                "string",
                "The temperature unit: 'celsius' or 'fahrenheit'",
                false,
            );

        temp_function.set_parameters(temp_params);

        // Create visibility function
        let mut vis_function = OllamaFunction::new(
            "get_current_visibility",
            "Get the current visibility conditions in a given location",
        );

        // Add parameters to visibility function
        let mut vis_params = OllamaFunctionParameters::new();
        vis_params
            .push_parameter(
                "location",
                "string",
                "The city and state, e.g., San Francisco, CA",
                true,
            )
            .push_parameter(
                "format",
                "string",
                "The visibility format: 'miles' or 'kilometers'",
                false,
            );

        vis_function.set_parameters(vis_params);

        // Add both functions to tools
        tools
            .push_function(temp_function)
            .push_function(vis_function);

        // Print the value of the structure
        println!(
            "---\ntools: {}",
            serde_json::to_string_pretty(&tools.array).unwrap()
        );

        // Verify the tools has two functions
        if let serde_json::Value::Array(functions) = &tools.array {
            assert_eq!(functions.len(), 2);

            // Check the type field in each function
            assert_eq!(functions[0]["type"], "function");
            assert_eq!(functions[1]["type"], "function");

            // Verify first function (temperature)
            if let serde_json::Value::Object(function0) = &functions[0]["function"] {
                assert_eq!(function0["name"], "get_current_temperature");
                assert_eq!(
                    function0["description"],
                    "Get the current temperature in a given location"
                );

                // Verify parameters were properly added
                if let serde_json::Value::Object(params) = &function0["parameters"] {
                    assert!(params.contains_key("properties"));
                    assert!(params.contains_key("required"));
                } else {
                    panic!("Expected parameters to be an object in first function");
                }
            } else {
                panic!("Expected function to be an object in first function");
            }

            // Verify second function (visibility)
            if let serde_json::Value::Object(function1) = &functions[1]["function"] {
                assert_eq!(function1["name"], "get_current_visibility");
                assert_eq!(
                    function1["description"],
                    "Get the current visibility conditions in a given location"
                );

                // Verify parameters were properly added
                if let serde_json::Value::Object(params) = &function1["parameters"] {
                    assert!(params.contains_key("properties"));
                    assert!(params.contains_key("required"));
                } else {
                    panic!("Expected parameters to be an object in second function");
                }
            } else {
                panic!("Expected function to be an object in second function");
            }
        } else {
            panic!("Expected tools to be an array");
        }
    }

    /// Tests the From<&serde_json::Value> implementation for OllamaToolCalls.
    ///
    /// This test verifies that:
    /// - An OllamaToolCalls can be created from a JSON array
    /// - An OllamaToolCalls can be created from a single JSON object (non-array)
    /// - Both conversions produce correctly structured tool calls
    #[test]
    fn test_from_value_for_tool_calls() {
        // Test creating OllamaToolCalls from a JSON array value
        let array_value = serde_json::json!([
            {
                "function": {
                    "arguments": {
                        "location": "New York"
                    },
                    "name": "get_weather"
                }
            },
            {
                "function": {
                    "arguments": {
                        "query": "Best restaurants"
                    },
                    "name": "search"
                }
            }
        ]);

        let tool_calls_from_array = OllamaToolCalls::from(&array_value);

        // Verify the conversion from array
        assert_eq!(tool_calls_from_array.len(), 2);
        assert_eq!(
            tool_calls_from_array.tool_call(0).unwrap().name(),
            Some("get_weather")
        );
        assert_eq!(
            tool_calls_from_array.tool_call(1).unwrap().name(),
            Some("search")
        );

        // Test creating OllamaToolCalls from a single JSON object value
        let object_value = serde_json::json!({
            "function": {
                "arguments": {
                    "message": "Hello world"
                },
                "name": "echo"
            }
        });

        let tool_calls_from_object = OllamaToolCalls::from(&object_value);

        // Verify the conversion from single object (should be wrapped in an array)
        assert_eq!(tool_calls_from_object.len(), 1);
        assert_eq!(
            tool_calls_from_object.tool_call(0).unwrap().name(),
            Some("echo")
        );

        // Confirm the JSON structure matches expectations
        let json_str = tool_calls_from_object.as_string_pretty();
        assert!(json_str.contains("echo"));
        assert!(json_str.contains("Hello world"));
    }
}

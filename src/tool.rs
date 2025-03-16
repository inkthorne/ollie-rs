//============================================================================
// OllamaFunctionParameters
//============================================================================
pub struct OllamaFunctionParameters {
    parameters: serde_json::Value,
}

impl OllamaFunctionParameters {
    pub fn new() -> Self {
        Self {
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }

    pub fn parameter(
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
        self.parameters["properties"][name] = details;

        // If parameter is required, add it to the required array
        if required {
            self.parameters["required"]
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
    function: serde_json::Value,
}

impl OllamaFunction {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            function: serde_json::json!({
                "name": name,
                "description": description,
            }),
        }
    }
    pub fn parameters(&mut self, parameters: OllamaFunctionParameters) -> &mut Self {
        self.function["parameters"] = parameters.parameters;
        self
    }
}

//============================================================================
// OllamaTools
//============================================================================
pub struct OllamaTools {
    tools: serde_json::Value,
}

impl OllamaTools {
    pub fn new() -> Self {
        Self {
            tools: serde_json::Value::Array(vec![]),
        }
    }

    pub fn add_function(&mut self, function: OllamaFunction) -> &mut Self {
        self.tools.as_array_mut().unwrap().push(function.function);
        self
    }

    /// Returns the underlying JSON value of the tools
    ///
    /// ## Returns
    ///
    /// A reference to the internal JSON value
    pub fn as_json(&self) -> &serde_json::Value {
        &self.tools
    }
}

//============================================================================
// TESTS
//============================================================================
#[cfg(test)]
mod tests {
    use super::*;

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
            .parameter("name", "string", "The name of the user", true)
            .parameter("email", "string", "The email address of the user", false);

        // Print the value of the structure
        println!(
            "---\nparameters: {}",
            serde_json::to_string_pretty(&params.parameters).unwrap()
        );

        // Basic assertion to verify the parameters were added
        if let serde_json::Value::Object(properties) = &params.parameters["properties"] {
            assert!(properties.contains_key("name"));
            assert!(properties.contains_key("email"));
        } else {
            panic!("Expected properties to be an object");
        }

        // Verify that only required parameters are in the required array
        if let serde_json::Value::Array(required) = &params.parameters["required"] {
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
            .parameter(
                "location",
                "string",
                "The city and state, e.g., San Francisco, CA",
                true,
            )
            .parameter(
                "format",
                "string",
                "The temperature unit to use: 'celsius' or 'fahrenheit'",
                false,
            );

        // Add parameters to the function
        weather_function.parameters(params);

        // Print the value of the structure
        println!(
            "---\n{}",
            serde_json::to_string_pretty(&weather_function.function).unwrap()
        );

        // Verify the function has the expected structure
        if let serde_json::Value::Object(function) = &weather_function.function {
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
            .parameter(
                "location",
                "string",
                "The city and state, e.g., San Francisco, CA",
                true,
            )
            .parameter(
                "unit",
                "string",
                "The temperature unit: 'celsius' or 'fahrenheit'",
                false,
            );

        temp_function.parameters(temp_params);

        // Create visibility function
        let mut vis_function = OllamaFunction::new(
            "get_current_visibility",
            "Get the current visibility conditions in a given location",
        );

        // Add parameters to visibility function
        let mut vis_params = OllamaFunctionParameters::new();
        vis_params
            .parameter(
                "location",
                "string",
                "The city and state, e.g., San Francisco, CA",
                true,
            )
            .parameter(
                "format",
                "string",
                "The visibility format: 'miles' or 'kilometers'",
                false,
            );

        vis_function.parameters(vis_params);

        // Add both functions to tools
        tools.add_function(temp_function).add_function(vis_function);

        // Print the value of the structure
        println!(
            "---\ntools: {}",
            serde_json::to_string_pretty(&tools.tools).unwrap()
        );

        // Verify the tools has two functions
        if let serde_json::Value::Array(functions) = &tools.tools {
            assert_eq!(functions.len(), 2);

            // Verify first function (temperature)
            assert_eq!(functions[0]["name"], "get_current_temperature");
            assert_eq!(
                functions[0]["description"],
                "Get the current temperature in a given location"
            );

            // Verify second function (visibility)
            assert_eq!(functions[1]["name"], "get_current_visibility");
            assert_eq!(
                functions[1]["description"],
                "Get the current visibility conditions in a given location"
            );

            // Verify parameters were properly added
            if let serde_json::Value::Object(params0) = &functions[0]["parameters"] {
                assert!(params0.contains_key("properties"));
                assert!(params0.contains_key("required"));
            } else {
                panic!("Expected parameters to be an object in first function");
            }

            if let serde_json::Value::Object(params1) = &functions[1]["parameters"] {
                assert!(params1.contains_key("properties"));
                assert!(params1.contains_key("required"));
            } else {
                panic!("Expected parameters to be an object in second function");
            }
        } else {
            panic!("Expected tools to be an array");
        }
    }
}

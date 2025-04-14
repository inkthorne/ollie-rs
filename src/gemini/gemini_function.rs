use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ===
// STRUCT: GeminiToolDeclaration
// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiToolDeclaration {
    #[serde(rename = "functionDeclarations", skip_serializing_if = "Vec::is_empty")]
    function_declarations: Vec<GeminiFunctionDeclaration>,
}

// ===
// PUBLIC: GeminiToolDeclaration
// ===

impl GeminiToolDeclaration {
    pub fn new() -> Self {
        Self {
            function_declarations: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: GeminiFunctionDeclaration) {
        let mut function = function;
        if let Some(object) = function.parameters.as_object_mut() {
            object.remove("$schema");
        }

        self.function_declarations.push(function);
    }
}

// ===
// STRUCT: GeminiFunctionParameters
// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiFunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: JsonValue,
}

// ===
// PUBLIC: GeminiFunctionParameters
// ===

impl GeminiFunctionDeclaration {
    pub fn new(name: &str, description: &str, parameters: RootSchema) -> Self {
        let parameters = serde_json::to_value(parameters).unwrap();

        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        }
    }
}

// ===
// TESTS: GeminiFunctionDeclaration
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Gemini;
    use crate::GeminiPromptUser;
    use crate::GeminiRequest;
    use schemars::JsonSchema;
    use schemars::schema_for;
    use std::env;

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct ScheduleMeetingParameters {
        /// The date of the meeting (e.g., '2023-10-01').
        pub date: String,

        /// The time of the meeting (e.g., '14:00').
        pub time: String,

        /// The topic of the meeting.
        pub topic: String,

        /// The list of people attending the meeting.
        pub attendees: Vec<String>,
    }

    #[test]
    fn test_gemini_function_declaration() {
        // Create a sample GeminiFunctionDeclaration
        let meeting_parameters = schema_for!(ScheduleMeetingParameters);
        let function_declaration = GeminiFunctionDeclaration {
            name: "schedule_meeting".to_string(),
            description: "Schedule a meeting with the given parameters.".to_string(),
            parameters: serde_json::to_value(meeting_parameters).unwrap(),
        };

        let pretty = serde_json::to_string_pretty(&function_declaration).unwrap();
        println!("{}", pretty);
    }

    #[test]
    fn test_gemini_tool_declaration() {
        // Create a sample GeminiToolDeclaration
        let meeting_parameters = schema_for!(ScheduleMeetingParameters);
        let function_declaration = GeminiFunctionDeclaration {
            name: "schedule_meeting".to_string(),
            description: "Schedule a meeting with the given parameters.".to_string(),
            parameters: serde_json::to_value(meeting_parameters).unwrap(),
        };

        let tool_declaration = GeminiToolDeclaration {
            function_declarations: vec![function_declaration],
        };

        let pretty = serde_json::to_string_pretty(&tool_declaration).unwrap();
        println!("{}", pretty);
    }

    #[tokio::test]
    async fn test_gemini_request_with_tools() {
        // Create the function declaration.
        let function_declaration = GeminiFunctionDeclaration::new(
            "schedule_meeting",                              // function name
            "Schedule a meeting with the given parameters.", // function description
            schema_for!(ScheduleMeetingParameters),          // function parameters schema
        );

        // Create the tool declaration.
        let mut tool_declaration = GeminiToolDeclaration::new();
        tool_declaration.add_function(function_declaration);

        // Create the user prompt.
        let prompt = GeminiPromptUser::new(
            "Schedule a meeting with Bob and Alice for 03/27/2025 at 10:00 AM about the Q3 planning.",
        );

        // Add the prompt & tool to the request.
        let mut request = GeminiRequest::from_prompt(&prompt);
        request.add_tool(tool_declaration);

        // Create the Gemini client.
        let api_key =
            env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

        // let model = "gemma-3-27b-it";
        let model = "gemini-2.0-flash";
        let gemini = Gemini::new(model, &api_key);

        // Send the request and get the response.
        let response = gemini.generate(&request).await.unwrap();

        println!("response: {}", response);
    }
}

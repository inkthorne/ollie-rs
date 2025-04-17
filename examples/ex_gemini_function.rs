use ollie_rs::{
    Gemini, GeminiFunctionCall, GeminiFunctionDeclaration, GeminiFunctionResponse,
    GeminiPromptUser, GeminiRequest, GeminiToolDeclaration,
};
use rand::Rng;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::env;

#[derive(Serialize, Deserialize, JsonSchema)]
struct SpecialNumberParameters {
    /// The base number to use in generating the special number.
    base: i32,
}

fn execute_function_call(function_call: &GeminiFunctionCall) -> GeminiFunctionResponse {
    match function_call.name() {
        "create_random_number" => {
            // Generate a random number between 1 and 100.
            let random_number = rand::rng().random_range(1..=100);
            GeminiFunctionResponse::new(
                "create_random_number",
                JsonValue::Number(random_number.into()),
            )
        }
        "create_special_number" => {
            // Simulate generating a special number based on the base number.
            let base: i32 = function_call.args().get("base").unwrap().as_i64().unwrap() as i32;
            let random_number = rand::rng().random_range(1..=100);
            let special_number = base * random_number;
            GeminiFunctionResponse::new(
                "create_special_number",
                JsonValue::Number(special_number.into()),
            )
        }
        _ => GeminiFunctionResponse::new("unknown_function", JsonValue::Null),
    }
}

#[tokio::main]
async fn main() {
    // Create the Gemini client.
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let model = "gemini-2.0-flash";
    let gemini = Gemini::new(model, &api_key);

    // Create the 'random number' function declaration.
    let fn_random_decl = GeminiFunctionDeclaration::build()
        .name("create_random_number")
        .description("Generates a random number between 1 and 100.");

    // Create the 'special number' function declaration.
    let fn_special_decl = GeminiFunctionDeclaration::build()
        .name("create_special_number")
        .description("Creates a special random number that is a multiple of the 'base' number.")
        .parameters(schema_for!(SpecialNumberParameters));

    // Create the tool declaration with the 2 functions.
    let mut tool_declaration = GeminiToolDeclaration::new();
    tool_declaration.add_function(fn_special_decl);
    tool_declaration.add_function(fn_random_decl);

    // Create the user prompt.
    let prompt = GeminiPromptUser::new(
        "Generate a random number, then use that random number as the base to generate a special number.",
    );

    // Add the prompt & tool to the request.
    let mut request = GeminiRequest::from_prompt(&prompt);
    request.add_tool(tool_declaration);

    // Send the request and get the response.
    let (mut request, response) = gemini.chat(request).await.unwrap();

    for function_call in response.functions() {
        let fn_response = execute_function_call(function_call);
        request.add_function_response(fn_response);
    }

    let (mut request, response) = gemini.chat(request).await.unwrap();

    for function_call in response.functions() {
        let fn_response = execute_function_call(function_call);
        request.add_function_response(fn_response);
    }

    let (_request, _response) = gemini.chat(request).await.unwrap();

    // println!("request: {}", _request);
    println!("response: {}", _response.text().unwrap());
}

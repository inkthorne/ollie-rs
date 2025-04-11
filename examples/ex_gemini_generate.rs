use ollie_rs::Gemini;
use ollie_rs::GeminiRequest;
use ollie_rs::GeminiResponse;
use std::env;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Create a new Gemini client with the API key.
    let gemini = Gemini::new(&api_key);

    // Choose a model - replace with an appropriate model available to your API key
    let model = "gemma-3-27b-it";

    // Create a request with a text prompt
    let request = GeminiRequest::text("Why is the sky blue?");

    println!("Sending request to Gemini API...");

    // Send the request and get the response.
    let response_json = gemini.generate(model, request.as_json()).await.unwrap();

    // Parse the JSON response into a GeminiResponse struct
    let response = GeminiResponse::new(response_json);

    // Extract and print just the text content from the response
    if let Some(text) = response.text() {
        println!("Response from Gemini:\n{}", text);
    }
}

use ollie_rs::Gemini;
use ollie_rs::GeminiResponse;
use ollie_rs::GeminiTextRequest;
use std::env;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Create a new Gemini client with the API key.
    let gemini = Gemini::new(&api_key);

    // Choose a model
    let model = "gemma-3-27b-it"; // Use an appropriate model that's available to you

    // Create a simple text request using the GeminiTextRequest helper.
    let request = GeminiTextRequest::new("Why is the sky blue?");

    println!("Sending request to Gemini API...");

    // Send the request and get the response.
    let response_json = gemini.generate(model, request.as_json()).await.unwrap();

    // Parse the JSON response into a GeminiResponse struct
    let response = GeminiResponse::new(response_json);

    // Extract and print just the text content
    if let Some(text) = response.text() {
        println!("Response from Gemini:\n{}", text);
    }
}

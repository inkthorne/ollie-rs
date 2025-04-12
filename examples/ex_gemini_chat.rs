use ollie_rs::Gemini;
use ollie_rs::GeminiRequest;
use ollie_rs::GeminiRole;
use std::env;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Choose a model - replace with an appropriate model available to your API key
    let model = "gemma-3-27b-it";

    // Create a new Gemini client with the API key.
    let gemini = Gemini::new(model, &api_key);

    // Create a new chat 'user' request.
    let request = GeminiRequest::from_prompt(GeminiRole::User, "Why is the sky blue?");

    println!("Sending request to Gemini API...");

    // Send the request and get the response.
    let response = gemini.generate(&request).await.unwrap();

    // Extract and print just the text content
    if let Some(text) = response.text() {
        println!("Response from Gemini:\n{}", text);
    }
}

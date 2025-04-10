use ollie_rs::{Gemini, GeminiResponse, GeminiTextRequest};
use std::env;
use std::io::Write;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Create a new Gemini client with the API key.
    let gemini = Gemini::new(&api_key);

    // Choose a model
    let model = "gemma-3-27b-it"; // Use an appropriate model that's available to you

    // Create a simple text request using the GeminiTextRequest helper.
    let request = GeminiTextRequest::new("Tell me a short story about a curious fox.");

    println!("Sending streaming request to Gemini API...");

    // Send the streaming request and get the response stream.
    let mut http_response = gemini
        .generate_stream(model, request.as_json())
        .await
        .unwrap();

    println!("Receiving streamed response:");

    // Process the streaming response using the provided method
    while let Some(json_response) = Gemini::read_stream(&mut http_response).await {
        let gemini_response = GeminiResponse::new(json_response);
        if let Some(text) = gemini_response.text() {
            print!("{}", text);
            std::io::stdout().flush().unwrap();
        }
    }

    println!("Streaming complete!");
}

use ollie_rs::{Gemini, GeminiRequest};
use std::env;
use std::io::Write;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable.
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Choose a model.
    let model = "gemma-3-27b-it";

    // Create the Gemini client using the 'model' and 'api_key'.
    let gemini = Gemini::new(model, &api_key);

    // Create a request with a text prompt.
    let request = GeminiRequest::from_str("Tell me a short story about a curious fox.");

    println!("\nSending streaming request to Gemini API...\n");

    // Send the request to generate a story.
    let mut stream = gemini.generate_stream(&request).await.unwrap();

    // Print the response as they arrive.
    while let Some(response) = stream.response().await {
        if let Some(text) = response.text() {
            print!("{}", text);
            std::io::stdout().flush().unwrap();
        }
    }
}

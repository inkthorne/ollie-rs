use ollie_rs::{Gemini, GeminiRequest};
use std::env;
use std::io::Write;

#[tokio::main]
async fn main() {
    // Create the Gemini client.
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let gemini = Gemini::new("gemini-2.0-flash", &api_key);

    println!("\nSending streaming request to Gemini API...\n");

    // Send the request to generate a story.
    let request = GeminiRequest::text("Tell me a short story about a curious fox.");
    let mut stream = gemini.generate_stream(request.as_json()).await.unwrap();

    // Print the response as they arrive.
    while let Some(response) = stream.response().await {
        if let Some(text) = response.text() {
            print!("{}", text);
            std::io::stdout().flush().unwrap();
        }
    }
}

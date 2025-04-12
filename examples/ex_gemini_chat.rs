use ollie_rs::Gemini;
use ollie_rs::GeminiRequest;
use ollie_rs::GeminiRole;
use std::env;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable.
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Choose a model.
    let model = "gemma-3-27b-it";

    // Create a new Gemini client with the API key.
    let gemini = Gemini::new(model, &api_key);

    // Create a new chat 'user' request.
    let mut request = GeminiRequest::from_prompt(GeminiRole::User, "Why is the sky blue?");

    // Send the 1st request and get the response.
    println!("Sending request #1 to Gemini API...");
    let response = gemini.chat(&mut request).await.unwrap();

    // Print just the text response.
    if let Some(text) = response.text() {
        println!("Response #1 from Gemini:\n{}", text);
    }

    // Add a new prompt to the request.
    request.add_prompt(
        GeminiRole::User,
        "Can you summarize what you just told me in 2 sentences?",
    );

    // Send the 2nd request and get the response.
    println!("Sending request #2 to Gemini API...");
    let response = gemini.chat(&mut request).await.unwrap();

    // Print just the text response.
    if let Some(text) = response.text() {
        println!("Response #2 from Gemini:\n{}", text);
    }
}

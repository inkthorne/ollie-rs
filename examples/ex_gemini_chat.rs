use ollie_rs::Gemini;
use ollie_rs::GeminiPromptUser;
use ollie_rs::GeminiRequest;
use std::env;

#[tokio::main]
async fn main() {
    // Get the API key from environment variable.
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Choose a model.
    let model = "gemma-3-27b-it";

    // Create a new Gemini client with the API key.
    let gemini = Gemini::new(model, &api_key);

    // Send the 1st request and get the response.
    println!("Sending request #1 to Gemini API...");

    let prompt_1 = GeminiPromptUser::new("Why is the sky blue?");
    let request = GeminiRequest::from_prompt(&prompt_1);
    let (mut request, response) = gemini.chat(request).await.unwrap();

    // Print just the text response.
    if let Some(text) = response.text() {
        println!("Response #1 from Gemini:\n{}", text);
    }

    // Add another prompt to the request.
    let prompt_2 = GeminiPromptUser::new("Can you summarize what you just told me in 2 sentences?");
    request.add_prompt(&prompt_2);

    // Send the 2nd request and get the response.
    println!("Sending request #2 to Gemini API...");
    let (_request, response) = gemini.chat(request).await.unwrap();

    // Print just the text response.
    if let Some(text) = response.text() {
        println!("Response #2 from Gemini:\n{}", text);
    }
}

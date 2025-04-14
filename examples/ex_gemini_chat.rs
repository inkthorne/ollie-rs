use ollie_rs::Gemini;
use ollie_rs::GeminiPromptUser;
use ollie_rs::GeminiRequest;
use std::env;

/// This example demonstrates how to have a multi-turn conversation with Gemini models
/// using the chat function. The conversation maintains context between turns.
#[tokio::main]
async fn main() {
    // === Setup ===
    // Get API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Initialize Gemini client with model
    let model = "gemma-3-27b-it";
    let gemini = Gemini::new(model, &api_key);

    // === First Turn: Initial Question ===
    println!("=== First Turn ===");

    // Create initial user prompt
    let initial_prompt = GeminiPromptUser::new("Why is the sky blue?");
    let request = GeminiRequest::from_prompt(&initial_prompt);

    // Send request and get response
    // Note: The chat function returns both the updated request (with conversation history)
    // and the model's response
    let (mut conversation, response) = gemini.chat(request).await.unwrap();

    // Display response
    if let Some(text) = response.text() {
        println!("Gemini: {}", text);
    }

    // === Second Turn: Follow-up Question ===
    println!("\n=== Second Turn ===");

    // Add a follow-up prompt to the same conversation
    let follow_up_prompt =
        GeminiPromptUser::new("Can you summarize what you just told me in 2 sentences?");
    conversation.add_prompt(&follow_up_prompt);

    // Send the updated conversation and get the new response
    let (_conversation, response) = gemini.chat(conversation).await.unwrap();

    // Display response
    if let Some(text) = response.text() {
        println!("Gemini: {}", text);
    }

    // The conversation could continue with more turns by adding more prompts
    // to the returned conversation object
}

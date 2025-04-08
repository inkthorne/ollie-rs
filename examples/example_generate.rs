use ollie_rs::Ollama;
use ollie_rs::request::OllamaRequest;
use std::io::{self, Write};

async fn simple_generate_example() {
    // Create a default Ollama client (connects to 127.0.0.1:11434)
    let ollama = Ollama::default();

    // Create and configure the request
    let mut request = OllamaRequest::new();
    request
        .set_model("gemma3:4b") // Use the model available on your Ollama server
        .set_prompt("Why is the sky blue?");

    request.prompt().map(|prompt| {
        println!("\nPrompt: {}\n", prompt);
    });

    // Send the request and handle the response
    let response = ollama
        .generate(&request, |response| {
            // Check if the response is an error
            if let Some(err) = response.error() {
                eprintln!("Error: {}", err);
                return;
            }

            // Extract the response text and print it
            response.response().map(|text| {
                print!("{}", text); // Print each chunk as it arrives
                io::stdout().flush().unwrap();
            });
        })
        .await
        .unwrap();

    response.map(|response| {
        println!("\n\ntokens used: {}", response.tokens_used());
    });

    println!("\n");
}

#[tokio::main]
async fn main() {
    simple_generate_example().await;
}

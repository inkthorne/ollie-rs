use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Create a new OllamaSession instance with the model to use
    let mut session = OllamaSession::new("gemma3:1b");

    // First, add the user message to the conversation
    let prompt = "Why is the sky blue?";
    println!("\n *** ASKING: {}\n", prompt);
    session.user(prompt);

    // Then call update() with just the callback to process the response
    session
        .update(|response| {
            print!("{}", response);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    // Add second user message to the conversation
    let prompt = "Could you summarize your previous response in a single sentence?";
    println!("\n\n *** ASKING: {}\n", prompt);
    session.user(prompt);

    // Call update() again to process the response
    let response = session
        .update(|content| {
            print!("{}", content);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    response.map(|text| {
        println!("\n\nSummary: {}", text.as_string_pretty());
    });

    println!("\n\nDone!");
}

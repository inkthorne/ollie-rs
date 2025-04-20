use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Create a new OllamaSession instance with the model to use
    let mut session = OllamaSession::local("gemma3:1b");

    // First, add the user message to the conversation
    let prompt = "Are swans always white?";
    println!("\n *** ASKING: {}\n", prompt);
    session.user(prompt);

    // Then call update() with just the callback to process the response
    let response = session
        .update(|response| {
            print!("{}", response);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    response.map(|response| {
        println!(
            "\n\n *** STATS: tokens used: {} of {}",
            response.tokens_used(),
            session.context_window_size()
        );
    });

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

    response.map(|response| {
        println!(
            "\n\n *** STATS: tokens used: {} of {}",
            response.tokens_used(),
            session.context_window_size()
        );
    });

    println!("\n");
}

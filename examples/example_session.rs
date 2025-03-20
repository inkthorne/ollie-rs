use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Create a new OllamaSession instance with the model to use
    let mut session = OllamaSession::new("gemma3:1b");

    // Use the prompt method with a callback that prints the response
    let prompt = "Why is the sky blue?";
    println!("\n *** ASKING: {}\n", prompt);
    session
        .prompt(prompt, |response| {
            print!("{}", response);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    // let prompt = "What was the previous question I asked you?";
    let prompt = "Could you summarize what your previous response in a single sentence?";
    println!("\n\n *** ASKING: {}\n", prompt);
    session
        .prompt(prompt, |response_text| {
            print!("{}", response_text);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    println!("\n\nDone!");
}

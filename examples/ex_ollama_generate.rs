use ollie_rs::{Ollama, OllamaRequest2, OllamaResponse2};
use std::io::Write;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let question = "Why is ocean water sometimes green?";

    let request = OllamaRequest2::new()
        .set_model("gemma3:1b")
        .set_prompt(question)
        .to_json();

    println!("\n-> question: {question}\n");

    let mut stream = ollama.generate2(&request).await.unwrap();

    // Handle the streamed responses as they arrive.
    while let Some(response_json) = stream.read().await {
        let response = OllamaResponse2::from_json(response_json).unwrap();

        // Print the error message, if any.
        if let Some(err) = response.error() {
            eprintln!("-> error: {}", err);
        }

        // Print the message of each response as it arrives.
        if let Some(text) = response.response() {
            print!("{}", text);
            std::io::stdout().flush().unwrap();
        }
    }

    // Print the response statistics.
    if let Ok(response) = OllamaResponse2::from_json(stream.response()) {
        response.print_stats();
    }
}

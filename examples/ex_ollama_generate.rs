use ollie_rs::{Ollama, OllamaRequest2};
use std::io::Write;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let question = "Why is ocean water sometimes green?";

    let request = OllamaRequest2::new()
        .set_model("gemma3:1b")
        .set_prompt(question);

    println!("\n-> question: {question}\n");

    let result = ollama
        .generate3(&request, |response| {
            // Print the error message, if any.
            if let Some(err) = response.error() {
                eprintln!("-> error: {}", err);
            }

            // Print each chunk as it arrives.
            if let Some(text) = response.text() {
                print!("{}", text);
                std::io::stdout().flush().unwrap();
            }
        })
        .await;

    if result.is_err() {
        eprintln!("-> error: {}", result.err().unwrap());
        return;
    }

    result.unwrap().print_stats();
}

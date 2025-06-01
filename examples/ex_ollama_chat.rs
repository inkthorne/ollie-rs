use ollie_rs::{Ollama, OllamaMessage2, OllamaOptions2, OllamaRequest2};
use std::io::Write;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let question = "Why is the sky blue?";

    let control = OllamaMessage2::new()
        .set_role("control")
        .set_content("thinking")
        .to_json();

    let user = OllamaMessage2::new()
        .set_role("user")
        .set_content(question)
        .to_json();

    let mut options = OllamaOptions2::new();
    options.set_num_ctx(8192).set_num_gpu(48);

    let mut request = OllamaRequest2::new();
    request
        // .model("granite3.3:8b")
        // .model("gemma3:12b")
        // .set_model("gemma3:4b")
        .set_model("gemma3:1b")
        .set_options(options.to_json())
        .add_message(control)
        .add_message(user);

    println!("\n-> question: {question}\n");

    // Send the chat request.
    let result = ollama
        .chat3(&request, |response| {
            // Print the error message, if any.
            if let Some(err) = response.error() {
                eprintln!("-> error: {}", err);
            }

            // Print the contents of each response as it arrives.
            if let Some(text) = response.text() {
                print!("{}", text);
                std::io::stdout().flush().unwrap();
            }
        })
        .await;

    // If an error occured, print it and abort.
    if result.is_err() {
        eprintln!("-> error: {}", result.err().unwrap());
        return;
    }

    // Print the response statistics.
    let response = result.unwrap();
    response.print_stats();

    // Ask a follow-up question based on the previous response.
    let question = "Can you summarize your previous answer in 2 sentences?";
    let user = OllamaMessage2::new()
        .set_role("user")
        .set_content(question)
        .to_json();

    // Add the response and the new user message to the previous request.
    request
        .add_response(&response)
        .add_message(user)
        .set_stream(false);

    println!("\n-> question: {question}\n");

    // Send the 2nd chat request.
    let result = ollama
        .chat3(&request, |response| {
            // Print the error message, if any.
            if let Some(err) = response.error() {
                eprintln!("-> error: {}", err);
            }

            // Print the contents of each response as it arrives.
            if let Some(text) = response.text() {
                print!("{}", text);
                std::io::stdout().flush().unwrap();
            }
        })
        .await;

    // If an error occured, print it and abort.
    if result.is_err() {
        eprintln!("-> error: {}", result.err().unwrap());
        return;
    }

    // Print the response statistics.
    let response = result.unwrap();
    response.print_stats();
}

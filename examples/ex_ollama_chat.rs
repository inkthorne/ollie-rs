use ollie_rs::{Ollama, OllamaMessage2, OllamaOptions2, OllamaRequest2, OllamaResponse2};
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

    let options = OllamaOptions2::new()
        .set_num_ctx(8192)
        .set_num_gpu(48)
        .to_json();

    let request = OllamaRequest2::new()
        // .model("granite3.3:8b")
        // .model("gemma3:12b")
        // .set_model("gemma3:4b")
        .set_model("gemma3:1b")
        .set_options(options)
        .add_message(control)
        .add_message(user)
        .to_json();

    println!("\n-> question: {question}\n");

    // Send the chat request.
    let mut stream = ollama.chat2(&request).await.unwrap();

    // Handle the streamed responses as they arrive.
    while let Some(response_json) = stream.read().await {
        let response = OllamaResponse2::from_json(response_json).unwrap();

        // Print the error message, if any.
        if let Some(err) = response.error() {
            eprintln!("-> error: {}", err);
        }

        // Print the message of each response as it arrives.
        if let Some(message) = response.message() {
            if let Some(content) = message.content() {
                print!("{}", content);
                std::io::stdout().flush().unwrap();
            }
        }
    }

    // Print the response statistics.
    if let Ok(response) = OllamaResponse2::from_json(stream.response()) {
        response.print_stats();
    }

    // Ask a follow-up question based on the previous response.
    let question = "Can you summarize your previous answer in 2 sentences?";
    let user = OllamaMessage2::new()
        .set_role("user")
        .set_content(question)
        .to_json();

    // Reuse the previous request and add the response and the new user message.
    let request = OllamaRequest2::from_json(request)
        .unwrap()
        .add_response(stream.response())
        .add_message(user)
        .set_stream(false)
        .to_json();

    println!("\n-> question: {question}\n");

    // Send the 2nd chat request.
    let mut stream = ollama.chat2(&request).await.unwrap();

    // Handle the non-streamed response.
    if let Some(response_json) = stream.read().await {
        let response = OllamaResponse2::from_json(response_json).unwrap();

        // Print the error message, if any.
        if let Some(err) = response.error() {
            eprintln!("-> error: {}", err);
        }

        // Print the message of each response as it arrives.
        if let Some(message) = response.message() {
            if let Some(content) = message.content() {
                print!("{}", content);
                std::io::stdout().flush().unwrap();
            }
        }

        // Print the response statistics.
        response.print_stats();
    }
}

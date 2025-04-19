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
    let mut stream = ollama.chat_json(&request).await.unwrap();

    // Handle the streamed responses as they arrive.
    while let Some(response) = stream.read().await {
        response["message"]["content"].as_str().map(|content| {
            print!("{}", content);
            std::io::stdout().flush().unwrap();
        });
    }

    // Print the response statistics.
    let response = OllamaResponse2::from_json(stream.response().clone()).unwrap();
    response.print_stats();
}

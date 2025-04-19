use ollie_rs::OllamaMessage2;
use ollie_rs::OllamaOptions2;
use ollie_rs::OllamaRequest2;
use ollie_rs::OllamaResponse;
use ollie_rs::ollama::Ollama;
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
        .set_model("gemma3:4b")
        // .model("gemma3:1b")
        .set_options(options)
        .add_message(control)
        .add_message(user)
        .to_json();

    println!("\n-> question: {question}\n");

    // Send the chat request and handle the response
    let mut stream = ollama.chat_json(&request).await.unwrap();

    while let Some(response) = stream.read().await {
        response["message"]["content"].as_str().map(|content| {
            print!("{}", content);
            std::io::stdout().flush().unwrap();
        });
    }

    let response = OllamaResponse::from_json(stream.response().clone());
    println!("\n\n-> tokens used: {}", response.tokens_used());
    println!("-> elapsed time (seconds): {:.1}", response.elapsed_time());
    println!("-> tokens/second: {:.1}", response.eval_rate());
}

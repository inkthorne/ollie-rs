use ollie_rs::OllamaMessageBuilder;
use ollie_rs::OllamaOptionsBuilder;
use ollie_rs::OllamaRequestBuilder;
use ollie_rs::OllamaResponse;
use ollie_rs::ollama::Ollama;
use std::io::Write;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let question = "Why is the sky blue?";

    let control = OllamaMessageBuilder::new()
        .role("control")
        .content("thinking")
        .build();

    let user = OllamaMessageBuilder::new()
        .role("user")
        .content(question)
        .build();

    let options = OllamaOptionsBuilder::new()
        .num_ctx(8192)
        .num_gpu(48)
        .build();

    let request = OllamaRequestBuilder::new()
        // .model("granite3.3:8b")
        // .model("gemma3:12b")
        .model("gemma3:4b")
        // .model("gemma3:1b")
        .options(options)
        .message(control)
        .message(user)
        .build();

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

use ollie_rs::ollama::Ollama;
use ollie_rs::request::OllamaRequest;
use std::error::Error;
use std::io::{self, Write};

async fn simple_generate_example() -> Result<(), Box<dyn Error>> {
    // Create a default Ollama client (connects to 127.0.0.1:11434)
    let ollama = Ollama::default();

    // Create and configure the request
    let mut request = OllamaRequest::new();
    request
        .set_model("gemma3:1b") // Use the model available on your Ollama server
        .set_prompt("Why is the sky blue?");

    request.prompt().map(|prompt| {
        println!("\nPrompt: {}\n", prompt);
    });

    // Send the request and handle the response
    ollama
        .generate(&request, |response| {
            // Check if the response is an error
            if let Some(err) = response.error() {
                eprintln!("Error: {}", err);
                return;
            }

            // Extract the response text and append it to our accumulated response
            response.response().map(|text| {
                print!("{}", text); // Print each chunk as it arrives
                io::stdout().flush().unwrap();
            });
        })
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    /*
    // Run the async function in a tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = simple_generate_example().await {
            eprintln!("Error: {}", e);
        }
    });
    */
    simple_generate_example().await.unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
    });
}

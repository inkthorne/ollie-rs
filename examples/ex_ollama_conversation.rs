use ollie_rs::OllamaSession;
use std::io::{self, Write};
// Uncomment to add delays between agent responses
// use tokio::time::{Duration, sleep};

/// This example demonstrates how to create two AI agents that have a conversation with each other
/// using Ollama sessions. Each agent responds to the other's messages in an infinite loop.
#[tokio::main]
async fn main() {
    // === Configuration ===
    const MODEL: &str = "gemma3:4b"; // Model to use for both agents
    const CONTEXT_WINDOW_SIZE: u32 = 24 * 1024; // Context window size

    // === Initialize Two Ollama Sessions ===
    // Create two separate Ollama sessions (agents)
    let mut agent1 = OllamaSession::local(MODEL);
    let mut agent2 = OllamaSession::local(MODEL);

    // Configure context window sizes
    agent1.set_context_window_size(CONTEXT_WINDOW_SIZE);
    agent2.set_context_window_size(CONTEXT_WINDOW_SIZE);

    // === Conversation Variables ===
    let mut agent1_response = String::new();
    // Initialize with a greeting to start the conversation
    let mut agent2_response = "Hello there! How are you doing today?".to_string();

    // === Begin Conversation Loop ===
    loop {
        // === Agent 1's Turn ("BOB") ===
        println!("\n\n=== AGENT 1 (BOB) ===\n");

        // Clear previous response and send Agent 2's message to Agent 1
        agent1_response.clear();
        agent1.user(&agent2_response);

        // Stream Agent 1's response and collect it
        let stats = agent1
            .update(|chunk| {
                // Print each chunk as it arrives
                print!("{}", chunk);
                io::stdout().flush().unwrap();
                // Store the complete response
                agent1_response.push_str(&chunk);
            })
            .await
            .unwrap();

        // Display token usage statistics
        println!(
            "\n\n[Stats] Tokens used: {} of {}",
            stats.tokens_used(),
            agent1.context_window_size()
        );

        // Optional: Add delay between responses
        // sleep(Duration::from_secs(10)).await;

        // === Agent 2's Turn ("FRED") ===
        println!("\n\n=== AGENT 2 (FRED) ===\n");

        // Clear previous response and send Agent 1's message to Agent 2
        agent2_response.clear();
        agent2.user(&agent1_response);

        // Stream Agent 2's response and collect it
        let stats = agent2
            .update(|chunk| {
                // Print each chunk as it arrives
                print!("{}", chunk);
                io::stdout().flush().unwrap();
                // Store the complete response
                agent2_response.push_str(&chunk);
            })
            .await
            .unwrap();

        // Display token usage statistics
        println!(
            "\n\n[Stats] Tokens used: {} of {}",
            stats.tokens_used(),
            agent2.context_window_size()
        );

        // Optional: Add delay between responses
        // sleep(Duration::from_secs(10)).await;
    }
}

# ollie-rs

A Rust library for interacting with large language models (LLMs) through simple, consistent APIs.

[![Crate](https://img.shields.io/crates/v/ollie-rs.svg)](https://crates.io/crates/ollie-rs)
[![Documentation](https://docs.rs/ollie-rs/badge.svg)](https://docs.rs/ollie-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **Note**: This crate is in active development. Some features may not be fully implemented or may be unstable. API changes are likely as the library evolves.

## Features

- **Ollama Integration**: Support for the [Ollama](https://ollama.ai) API, including:
  - Text generation
  - Chat completion
  - Function calling / tools
  - Streaming responses
  - Session management for multi-turn conversations
  
- **Gemini Integration**: Support for Google's [Gemini](https://deepmind.google/technologies/gemini/) AI models, including:
  - Text generation
  - Chat completion
  - Function calling / tools
  - Streaming responses

- **Modern Rust APIs**:
  - Async-first design with Tokio
  - Builder pattern for request construction
  - Strongly typed interfaces
  - Callback-based streaming

## Quick Start

Add ollie-rs to your project:

```bash
cargo add ollie-rs
```

### Example: Simple Chat with Ollama

```rust
use ollie_rs::Ollama;
use ollie_rs::OllamaMessage;
use ollie_rs::OllamaOptions;
use ollie_rs::OllamaRequest;

#[tokio::main]
async fn main() {
    // Create a default Ollama client (connects to 127.0.0.1:11434)
    let ollama = Ollama::default();
    
    // Optional: Configure parameters
    let mut options = OllamaOptions::new();
    options.set_temperature(0.7);
    
    // Create a message
    let mut message = OllamaMessage::new();
    message.set_role("user").set_content("Why is the sky blue?");
    
    // Create and configure the request
    let mut request = OllamaRequest::new();
    request
        .set_model("llama3") // Use any model available on your Ollama server
        .set_options(&options)
        .add_message(&message);
    
    // Send the chat request and handle the response
    ollama
        .chat(&request, |response| {
            // Print each chunk as it arrives
            response.content().map(|text| print!("{}", text));
        })
        .await
        .unwrap();
}
```

### Example: Using Chat Sessions

Sessions provide a convenient way to maintain conversation history:

```rust
use ollie_rs::OllamaSession;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Create a new chat session with a specified model
    let mut session = OllamaSession::new("llama3:8b");
    
    // Add a system message to set the assistant's behavior
    session.system("You are a helpful AI assistant who responds concisely.");
    
    // Add a user message
    session.user("Tell me about Rust programming language.");
    
    // Get the response
    session.update(|chunk| {
        // Process each chunk of the response as it arrives
        print!("{}", chunk);
        io::stdout().flush().unwrap();
    }).await.unwrap();
    
    // Continue the conversation
    session.user("What are some popular crates?");
    
    // Get the next response
    session.update(|chunk| {
        print!("{}", chunk);
        io::stdout().flush().unwrap();
    }).await.unwrap();
}
```

## Function Calling / Tools

ollie-rs supports function calling (tools) with Ollama models that have this capability:

```rust
use ollie_rs::{Ollama, OllamaFunction, OllamaFunctionParameters, OllamaMessage, OllamaRequest, OllamaTools};

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    
    // Define a function
    let mut tools = OllamaTools::new();
    let mut weather_function = OllamaFunction::new(
        "get_current_weather",
        "Gets the current weather for a location."
    );
    
    // Add parameters to the function
    let mut params = OllamaFunctionParameters::new();
    params.push_parameter(
        "location", 
        "string", 
        "The city and state, e.g., San Francisco, CA", 
        true
    );
    
    weather_function.set_parameters(params);
    tools.push_function(weather_function);
    
    // Create a message that might trigger tool use
    let mut message = OllamaMessage::new();
    message.set_role("user").set_content("What's the weather like in Paris?");
    
    // Create the request with tools
    let mut request = OllamaRequest::new();
    request
        .set_model("llama3")
        .set_tools(&tools)
        .add_message(&message);
    
    // Handle the response
    ollama.chat(&request, |response| {
        // Process tool calls or regular responses
        if let Some(message) = response.message() {
            if let Some(tool_calls) = message.tool_calls() {
                for i in 0..tool_calls.len() {
                    let tool_call = tool_calls.tool_call(i).unwrap();
                    // Process the tool call (e.g., get actual weather data)
                    println!("Tool called: {}", tool_call.name().unwrap());
                }
            }
        }
        
        // Print regular response content
        if let Some(content) = response.content() {
            print!("{}", content);
        }
    }).await.unwrap();
}
```

## Configuration

### Custom Ollama Server

By default, ollie-rs connects to an Ollama server at `127.0.0.1:11434`. You can:

1. Set the `OLLAMA_SERVER` environment variable:
   ```bash
   export OLLAMA_SERVER=192.168.1.100:11434
   ```

2. Specify a custom server in code:
   ```rust
   let ollama = Ollama::new("192.168.1.100:11434");
   // or
   let session = OllamaSession::remote("llama3", "192.168.1.100:11434");
   ```

## Examples

Check the `examples/` directory for more detailed examples:
- `ex_ollama_chat.rs`: Basic chat interaction
- `ex_ollama_generate.rs`: Text generation
- `ex_ollama_session.rs`: Multi-turn conversation using sessions
- `ex_ollama_conversation.rs`: Interactive conversation example
- `ex_gemini_generate.rs`: Text generation with Gemini
- `ex_gemini_generate_stream.rs`: Streamed text generation with Gemini
- `ex_gemini_chat.rs`: Basic chat interaction with Gemini
- `ex_gemini_function.rs`: Function calling with Gemini

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ollie-rs = "0.1.0"
```

## Requirements

- Rust 2024 Edition or newer
- [Ollama](https://ollama.ai) running locally or on an accessible server (for Ollama features)
- Google AI API Key (for Gemini features)

## License

This project is licensed under the MIT License - see the LICENSE file for details.
use crate::{
    Ollama, OllamaMessage2, OllamaOptions2, OllamaRequest2, OllamaResponse, OllamaResponse2,
};

// ===
// STRUCT: OllamaSession
// ===

/// A session for interacting with Ollama chat models.
///
/// This struct manages the state of a conversation with an Ollama model,
/// keeping track of the message history for context in future exchanges.
pub struct OllamaSession {
    ollama: Ollama,
    request: OllamaRequest2,
    options: OllamaOptions2,
}

impl OllamaSession {
    /// Creates a new chat session with the specified model.
    ///
    /// This method checks for the OLLIE_HOST environment variable. If set, it connects
    /// to the server address specified in the environment variable. Otherwise, it
    /// connects to the default local Ollama server (127.0.0.1:11434).
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the Ollama model to use for this chat session.
    ///
    /// # Returns
    ///
    /// A new `OllamaSession` instance configured to use the specified model.
    pub fn new(model: &str) -> Self {
        match std::env::var("OLLAMA_SERVER") {
            Ok(host) => Self::remote(model, &host),
            Err(_) => Self::local(model),
        }
    }

    /// Creates a new chat session with the specified model using the local Ollama server.
    ///
    /// This method connects to the default local Ollama server address (127.0.0.1:11434).
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the Ollama model to use for this chat session.
    ///
    /// # Returns
    ///
    /// A new `OllamaSession` instance configured to use the specified model with the local server.
    pub fn local(model: &str) -> Self {
        let request = OllamaRequest2::new().set_model(model);
        let ollama = Ollama::default();

        OllamaSession {
            ollama,
            request,
            options: OllamaOptions2::new(),
        }
    }

    /// Creates a new chat session with the specified model.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the Ollama model to use for this chat session.
    /// * `server_address` - The server address (e.g., "127.0.0.1:11434") where the Ollama server is running.
    ///
    /// # Returns
    ///
    /// A new `OllamaChat` instance configured to use the specified model.
    pub fn remote(model: &str, server_address: &str) -> Self {
        let ollama = Ollama::new(server_address);
        let request = OllamaRequest2::new().set_model(model);

        OllamaSession {
            ollama,
            request,
            options: OllamaOptions2::new(),
        }
    }

    /// Adds an assistant message to the conversation.
    ///
    /// Assistant messages represent responses from the AI assistant
    /// and are included in the conversation history.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the assistant message.
    pub fn assistant(&mut self, content: &str) {
        let message = OllamaMessage2::new()
            .set_role("assistant")
            .set_content(content)
            .to_json();
        self.request = self.request.clone().add_message(message);
    }

    /// Gets the context window size for the model.
    ///
    /// Returns the number of tokens that can be processed in a single request.
    /// If not explicitly set in options, defaults to 2048 tokens.
    ///
    /// # Returns
    ///
    /// The context window size as a u32 value.
    pub fn context_window_size(&self) -> u32 {
        self.options.num_ctx().unwrap_or(2048)
    }

    pub fn set_context_window_size(&mut self, num_ctx: u32) {
        self.options = self.options.clone().set_num_ctx(num_ctx);
    }

    /// Gets a mutable reference to the options for configuring model behavior.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `OllamaOptions2` instance.
    pub fn options(&mut self) -> &mut OllamaOptions2 {
        &mut self.options
    }

    /// Adds a user message to the conversation.
    ///
    /// User messages represent queries or statements from the user
    /// and are included in the conversation history.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the user message.
    pub fn user(&mut self, content: &str) {
        let message = OllamaMessage2::new()
            .set_role("user")
            .set_content(content)
            .to_json();

        self.request = self.request.clone().add_message(message);
    }

    /// Adds a system message to the conversation.
    ///
    /// System messages provide instructions or context to the model
    /// about how it should behave throughout the conversation.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the system message.
    pub fn system(&mut self, content: &str) {
        let message = OllamaMessage2::new()
            .set_role("system")
            .set_content(content)
            .to_json();

        self.request = self.request.clone().add_message(message);
    }

    /// Sends a prompt to the model and processes the response.
    ///
    /// This method sends the prompt to the Ollama model, processes the
    /// streaming response, and updates the conversation history.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The text prompt to send to the model.
    /// * `callback` - A function that will be called with each chunk of the response
    ///    as it is received. Use this for handling streaming responses.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the prompt was processed successfully.
    /// * `Err(String)` - If an error occurred, containing the error message.
    pub async fn update<F>(&mut self, mut callback: F) -> Result<Option<OllamaResponse2>, String>
    where
        F: FnMut(&str),
    {
        // Apply options to the request
        self.request = self
            .request
            .clone()
            .set_options(self.options.clone().to_json());

        self.request = self.request.clone().set_stream(true);

        let request = self.request.clone().to_json();

        match self.ollama.chat2(&request).await {
            Ok(mut stream) => {
                stream.save_responses(true);

                while let Some(response_json) = stream.read().await {
                    let response = OllamaResponse::from_json(response_json);

                    // Extract the response content and pass it to the callback if available
                    if let Some(message) = response.message() {
                        if let Some(content) = message.content() {
                            callback(content);
                        }
                    }
                }

                let response = OllamaResponse2::from_json(stream.response()).ok();
                Ok(response)
            }

            Err(e) => Err(e.to_string()),
        }
    }
}

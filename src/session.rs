use crate::ollama::Ollama;
use crate::{message::OllamaMessage, option::OllamaOptions, request::OllamaRequest};

//============================================================================
// OllamaSession
//============================================================================
/// A session for interacting with Ollama chat models.
///
/// This struct manages the state of a conversation with an Ollama model,
/// keeping track of the message history for context in future exchanges.
pub struct OllamaSession {
    ollama: Ollama,
    request: OllamaRequest,
    options: OllamaOptions,
}

impl OllamaSession {
    /// Creates a new chat session with the specified model.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the Ollama model to use for this chat session.
    ///
    /// # Returns
    ///
    /// A new `OllamaChat` instance configured to use the specified model.
    pub fn new(model: &str) -> Self {
        let mut request = OllamaRequest::new();
        request.set_model(model);

        OllamaSession {
            ollama: Ollama::default(),
            request,
            options: OllamaOptions::new(),
        }
    }

    /// Gets a mutable reference to the options for configuring model behavior.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `OllamaOptions` instance.
    pub fn options(&mut self) -> &mut OllamaOptions {
        &mut self.options
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
        let mut message = OllamaMessage::new();
        message.set_role("assistant").set_content(content);
        self.request.push_message(&message);
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
        let mut message = OllamaMessage::new();
        message.set_role("user").set_content(content);
        self.request.push_message(&message);
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
        let mut message = OllamaMessage::new();
        message.set_role("system").set_content(content);
        self.request.push_message(&message);
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
    pub async fn update<F>(&mut self, mut callback: F) -> Result<(), String>
    where
        F: FnMut(&str),
    {
        // Apply options to the request
        self.request.set_options(&self.options);

        // Accumulate all the content from responses
        let mut accumulated_content = String::new();

        match self
            .ollama
            .chat(&self.request, |response| {
                response.error().map(|err| {
                    // If there's an error, call the callback with the error message
                    callback(err);
                });

                // Extract the response content and pass it to the callback if available
                if let Some(content) = response.content() {
                    callback(content);
                    // Accumulate the content
                    accumulated_content.push_str(content);
                }
            })
            .await
        {
            Ok(_) => {
                // Create an assistant message with the accumulated content
                let mut assistant_message = OllamaMessage::new();
                assistant_message
                    .set_role("assistant")
                    .set_content(&accumulated_content);

                // Add the assistant message to the request for context in future exchanges
                self.request.push_message(&assistant_message);

                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

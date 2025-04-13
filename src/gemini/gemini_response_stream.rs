use crate::GeminiResponse;
use reqwest::Response as HttpResponse;
use serde_json::Value as JsonValue;

/// A stream for processing Gemini API responses.
///
/// This struct wraps an HTTP response and provides methods to parse and extract
/// Gemini response data from the server-sent event (SSE) format.
pub struct GeminiResponseStream {
    http_response: HttpResponse,
    responses: Vec<GeminiResponse>,
}

impl GeminiResponseStream {
    /// Creates a new GeminiResponseStream from an HTTP response.
    ///
    /// # Arguments
    /// * `http_response` - The HTTP response to wrap
    ///
    /// # Returns
    /// * A new GeminiResponseStream instance
    pub fn new(http_response: HttpResponse) -> Self {
        GeminiResponseStream {
            http_response,
            responses: Vec::new(),
        }
    }

    /// Fetches and parses the next chunk of data from the stream.
    ///
    /// This method retrieves the next chunk from the HTTP response, parses it as an SSE message,
    /// and converts it to a `GeminiResponse` object.
    ///
    /// # Returns
    /// * `Some(GeminiResponse)` if a valid response chunk was received and parsed
    /// * `None` if the stream has ended or an error occurred during parsing
    pub async fn read(&mut self) -> Option<GeminiResponse> {
        let bytes = self.http_response.chunk().await.ok()?;

        if bytes.is_none() {
            return None;
        }

        let bytes = bytes.unwrap();
        let string = String::from_utf8(bytes.to_vec()).ok()?;
        let slice = string.split_once("data:")?.1;
        let value: JsonValue = serde_json::from_str(&slice).ok()?;
        let response = GeminiResponse::new(value);

        // Save the response
        self.responses.push(response.clone());

        Some(response)
    }

    /// Returns a reference to the stored responses that have been collected from the stream.
    ///
    /// This method allows accessing all the response objects that have been
    /// received from the stream so far.
    ///
    /// # Returns
    /// * A reference to the vector of GeminiResponse objects
    pub fn responses(&self) -> &Vec<GeminiResponse> {
        &self.responses
    }

    /// Concatenates the text content from all responses into a single String.
    ///
    /// This method iterates through all collected responses and combines
    /// their text content sequentially.
    ///
    /// # Returns
    /// * A String containing the combined text from all responses
    pub fn text(&self) -> String {
        self.responses
            .iter()
            .filter_map(|response| response.text())
            .collect::<Vec<&str>>()
            .join("")
    }
}

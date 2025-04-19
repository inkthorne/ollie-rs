use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Clone)]
pub struct OllamaOptions2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_ctx: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_gpu: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

impl OllamaOptions2 {
    /// Creates a new `OllamaOptions2` instance with all options set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    ///
    /// let options = OllamaOptions2::new();
    /// ```
    pub fn new() -> Self {
        Self {
            temperature: None,
            num_gpu: None,
            num_predict: None,
            num_ctx: None,
        }
    }

    /// Deserializes an `OllamaOptions2` from a `serde_json::Value`.
    ///
    /// # Arguments
    ///
    /// * `json` - A `serde_json::Value` representing the options.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `Ok(Self)` - Successfully deserialized OllamaOptions2 instance
    /// - `Err(serde_json::Error)` - If deserialization fails
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    /// use serde_json::json;
    ///
    /// let json_data = json!({
    ///     "num_ctx": 4096,
    ///     "temperature": 0.8
    /// });
    ///
    /// let options = OllamaOptions2::from_json(json_data).unwrap();
    /// assert_eq!(options.num_ctx(), Some(4096));
    /// assert_eq!(options.temperature(), Some(0.8));
    /// ```
    pub fn from_json(json: JsonValue) -> Result<Self, serde_json::Error> {
        let options = serde_json::from_value(json)?;
        Ok(options)
    }

    /// Converts this `OllamaOptions2` instance to a `serde_json::Value`.
    ///
    /// This method serializes the options into a JSON value, skipping any fields
    /// that are set to `None`.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the options.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    /// use serde_json::json;
    ///
    /// let options = OllamaOptions2::new().set_num_ctx(2048).set_temperature(0.7);
    /// let json_val = options.to_json();
    ///
    /// assert_eq!(json_val, json!({
    ///     "num_ctx": 2048,
    ///     "temperature": 0.7
    /// }));
    /// ```
    pub fn to_json(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }

    /// Returns the number of context tokens, or `None` if not set.
    ///
    /// # Returns
    ///
    /// An `Option<i32>` containing the number of context tokens if set, otherwise `None`.
    pub fn num_ctx(&self) -> Option<i32> {
        self.num_ctx
    }

    /// Sets the number of context tokens.
    ///
    /// # Arguments
    ///
    /// * `num_ctx` - The number of context tokens to use.
    ///
    /// # Returns
    ///
    /// Self with the updated value for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    ///
    /// let options = OllamaOptions2::new().set_num_ctx(2048);
    /// assert_eq!(options.num_ctx(), Some(2048));
    /// ```
    pub fn set_num_ctx(mut self, num_ctx: i32) -> Self {
        self.num_ctx = Some(num_ctx);
        self
    }

    /// Returns the number of LLM layers used on the GPUs, or `None` if not set.
    ///
    /// # Returns
    ///
    /// An `Option<i32>` containing the number of GPUs if set, otherwise `None`.
    pub fn num_gpu(&self) -> Option<i32> {
        self.num_gpu
    }

    /// Sets the number of LLM layers to push to the GPU.
    ///
    /// # Arguments
    ///
    /// * `num_gpu` - The number of LLM layers to push to the GPU.
    ///
    /// # Returns
    ///
    /// Self with the updated value for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    ///
    /// let options = OllamaOptions2::new().set_num_gpu(1);
    /// assert_eq!(options.num_gpu(), Some(1));
    /// ```
    pub fn set_num_gpu(mut self, num_gpu: i32) -> Self {
        self.num_gpu = Some(num_gpu);
        self
    }

    /// Returns the number of tokens to predict, or `None` if not set.
    ///
    /// # Returns
    ///
    /// An `Option<i32>` containing the number of tokens to predict if set, otherwise `None`.
    pub fn num_predict(&self) -> Option<i32> {
        self.num_predict
    }

    /// Sets the number of tokens to predict.
    ///
    /// # Arguments
    ///
    /// * `num_predict` - The number of tokens to predict.
    ///
    /// # Returns
    ///
    /// Self with the updated value for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    ///
    /// let options = OllamaOptions2::new().set_num_predict(100);
    /// assert_eq!(options.num_predict(), Some(100));
    /// ```
    pub fn set_num_predict(mut self, num_predict: i32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    /// Returns the temperature value, or `None` if not set.
    ///
    /// # Returns
    ///
    /// An `Option<f32>` containing the temperature value if set, otherwise `None`.
    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    /// Sets the temperature value.
    ///
    /// # Arguments
    ///
    /// * `temperature` - The temperature value to use for generation.
    ///   Higher values make output more random, lower values more deterministic.
    ///
    /// # Returns
    ///
    /// Self with the updated value for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use ollie_rs::OllamaOptions2;
    ///
    /// let options = OllamaOptions2::new().set_temperature(0.7);
    /// assert_eq!(options.temperature(), Some(0.7));
    /// ```
    pub fn set_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

// ===
// TESTS: OllamaOptions2
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let options = OllamaOptions2::new();
        // Check internal state is None
        assert!(options.num_ctx.is_none());
        assert!(options.num_gpu.is_none());
        assert!(options.num_predict.is_none());
        assert!(options.temperature.is_none());
    }

    #[test]
    fn test_accessors() {
        let options = OllamaOptions2::new()
            .set_num_ctx(2048)
            .set_num_gpu(1)
            .set_num_predict(100)
            .set_temperature(0.7);

        assert_eq!(options.num_ctx(), Some(2048));
        assert_eq!(options.num_gpu(), Some(1));
        assert_eq!(options.num_predict(), Some(100));
        assert_eq!(options.temperature(), Some(0.7));
    }

    #[test]
    fn test_from_json_valid() {
        let json_data = json!({
            "num_ctx": 4096,
            "num_gpu": 2,
            "num_predict": 256,
            "temperature": 0.8
        });
        let options_result = OllamaOptions2::from_json(json_data);
        assert!(options_result.is_ok());
        let options = options_result.unwrap();
        assert_eq!(options.num_ctx(), Some(4096));
        assert_eq!(options.num_gpu(), Some(2));
        assert_eq!(options.num_predict(), Some(256));
        assert_eq!(options.temperature(), Some(0.8));
    }

    #[test]
    fn test_from_json_invalid_type() {
        let json_data = json!({
            "num_ctx": "invalid", // String instead of i32
            "temperature": 0.8
        });
        let options_result = OllamaOptions2::from_json(json_data);
        assert!(options_result.is_err());
    }

    #[test]
    fn test_from_json_partial() {
        let json_data = json!({
            "num_ctx": 4096,
            "temperature": 0.8
            // Missing other fields
        });

        let options_result = OllamaOptions2::from_json(json_data);
        assert!(options_result.is_ok());

        let options = options_result.unwrap();
        assert_eq!(options.num_ctx(), Some(4096));
        assert_eq!(options.temperature(), Some(0.8));
        assert_eq!(options.num_gpu(), None);
        assert_eq!(options.num_predict(), None);
    }

    #[test]
    fn test_to_json() {
        let options = OllamaOptions2::new().set_num_ctx(2048).set_temperature(0.7);

        let json_val = options.to_json();

        // Extract values to compare individually
        let json_num_ctx = json_val["num_ctx"].as_i64().unwrap() as i32;
        let json_temp = json_val["temperature"].as_f64().unwrap() as f32;

        assert_eq!(json_num_ctx, 2048);
        assert!((json_temp - 0.7).abs() < f32::EPSILON);

        // Make sure no extra fields are present
        assert_eq!(json_val.as_object().unwrap().len(), 2);
        assert!(json_val.get("num_gpu").is_none());
        assert!(json_val.get("num_predict").is_none());
    }

    #[test]
    fn test_to_json_empty() {
        let options = OllamaOptions2::new();
        let json_val = options.to_json();
        // Because of skip_serializing_if, empty fields should not be present
        let expected_json = json!({});
        assert_eq!(json_val, expected_json);
    }
}

use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize, Clone)]
pub struct OllamaOptionsBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_ctx: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_gpu: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

impl OllamaOptionsBuilder {
    pub fn new() -> Self {
        Self {
            temperature: None,
            num_gpu: None,
            num_predict: None,
            num_ctx: None,
        }
    }

    pub fn build(self) -> JsonValue {
        serde_json::to_value(self).unwrap()
    }

    pub fn num_ctx(mut self, num_ctx: i32) -> Self {
        self.num_ctx = Some(num_ctx);
        self
    }

    pub fn num_gpu(mut self, num_gpu: i32) -> Self {
        self.num_gpu = Some(num_gpu);
        self
    }

    pub fn num_predict(mut self, num_predict: i32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

// chat_request.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<MessageOpenAI>,
    pub model: String,
    pub frequency_penalty: Option<f64>,
    pub logit_bias: Option<Vec<(String, f64)>>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u32>,
    pub max_tokens: Option<u32>,
    pub n: Option<u32>,
    pub presence_penalty: Option<f64>,
    pub response_format: Option<Format>,
    pub seed: Option<u64>,
    pub stop: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageOpenAI {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    #[serde(rename = "type")]
    pub format_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub tool_choice: Option<ToolChoice>,
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolChoice {
    #[serde(rename = "type")]
    pub choice_type: String,
    pub function: Option<Function>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
}
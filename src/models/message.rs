use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: String,
    pub id: String,
    pub timestamp: String,
    pub text: TextBody,
    #[serde(rename = "type")]
    pub message_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TextBody {
    pub body: String,
}

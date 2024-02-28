use serde::Deserialize;
use super::Entry; // Importação relativa dentro do módulo models

#[derive(Debug, Deserialize)]
pub struct WebhookData {
    pub object: String,
    pub entry: Vec<Entry>,
}
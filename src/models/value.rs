use serde::Deserialize;
use super::{Metadata, Contact, Message}; // Importações relativas

#[derive(Debug, Deserialize)]
pub struct Value {
    pub messaging_product: String,
    pub metadata: Metadata,
    pub contacts: Vec<Contact>,
    pub messages: Vec<Message>,
}

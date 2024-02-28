use serde::Deserialize;
use super::Value; // Importação relativa

#[derive(Debug, Deserialize)]
pub struct Change {
    pub value: Value,
    pub field: String,
}

use serde::Deserialize;
use super::Change; // Importação relativa dentro do módulo models

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

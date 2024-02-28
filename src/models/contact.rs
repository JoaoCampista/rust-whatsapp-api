use serde::Deserialize;
use super::Profile; // Importação relativa dentro do módulo models

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub profile: Profile,
    pub wa_id: String,
}

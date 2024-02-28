use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub display_phone_number: String,
    pub phone_number_id: String,
}

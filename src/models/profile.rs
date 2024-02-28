use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
}

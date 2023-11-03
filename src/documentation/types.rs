use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DocResult {
    pub title: String,
    pub url: String,
}

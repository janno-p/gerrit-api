use serde::Deserialize;

use crate::{GerritClient, client::GerritError};

#[derive(Debug, Deserialize)]
pub struct DocResult {
    pub title: String,
    pub url: String,
}

pub struct SearchBuilder {
    pub(crate) query: String,
}

impl SearchBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<Vec<DocResult>, GerritError> {
        client.query(format!("Documentation/?q={}", self.query)).await
    }
}

pub fn search(query: String) -> SearchBuilder {
    SearchBuilder { query }
}

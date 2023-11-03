mod types;

pub use types::*;

use crate::{GerritClient, client::GerritError};

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

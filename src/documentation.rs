use reqwest::StatusCode;
use serde::Deserialize;

use crate::GerritClient;

#[derive(Debug, Deserialize)]
pub struct DocResult {
    pub title: String,
    pub url: String,
}

pub struct SearchBuilder {
    pub(crate) query: String,
}

impl SearchBuilder {
    pub async fn execute(&self, api: &GerritClient) -> Result<Vec<DocResult>, StatusCode> {
        let result = api.get(format!("Documentation/?q={}", self.query)).send().await;

        match result {
            Ok(response) if response.status() == StatusCode::OK => {
                match response.text().await {
                    Ok(v) => serde_json::from_str::<Vec<DocResult>>(&v[4..]).map_err(|_| StatusCode::BAD_REQUEST),
                    Err(_) => Err(StatusCode::BAD_REQUEST),
                }
            },
            Ok(response) => Err(response.status()),
            Err(e) => Err(e.status().unwrap_or(StatusCode::BAD_REQUEST)),
        }
    }
}

pub fn search<'a>(query: String) -> SearchBuilder {
    SearchBuilder { query }
}

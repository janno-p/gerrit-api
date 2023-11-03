use reqwest::{Client, StatusCode};
use serde::de;

pub struct GerritClient {
    http_client: Client,
    base_url: String,
    account_token: String,
}

#[derive(Debug)]
pub enum GerritError {
    InvalidContentType(String),
    InvalidJson,
    InvalidStatusCode(StatusCode),
    InvalidRequest(reqwest::Error),
}

impl GerritClient {
    pub fn new(base_url: String, account_token: String) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
            account_token,
        }
    }

    pub(crate) async fn query<T>(&self, url: String) -> Result<T, GerritError>
    where
        T: de::DeserializeOwned,
    {
        println!("==> {}/{}", self.base_url, url);

        let result = self
            .http_client
            .get(format!("{}/{}", self.base_url, url))
            .header("Cookie", format!("GerritAccount={}", &self.account_token))
            .send()
            .await;

        match result {
            Ok(response) if response.status() == StatusCode::OK => {
                if let Some(content_type) = response.headers().get("content-type") {
                    let content_type = String::from_utf8(content_type.as_bytes().to_vec()).unwrap_or_default();
                    if let Some("application/json") = content_type.split(";").next() {
                        let text = response.text().await;
                        match text {
                            Ok(json_string) => serde_json::from_str(&json_string[4..]).map_err(|_| GerritError::InvalidJson),
                            Err(_) => Err(GerritError::InvalidJson),
                        }
                    } else {
                        Err(GerritError::InvalidContentType(content_type))
                    }
                } else {
                    Err(GerritError::InvalidContentType("".to_string()))
                }
            },
            Ok(response) => Err(GerritError::InvalidStatusCode(response.status())),
            Err(err) => {
                err.status()
                .map(|status_code| Err(GerritError::InvalidStatusCode(status_code)))
                .unwrap_or(Err(GerritError::InvalidRequest(err)))
            },
        }
    }
}

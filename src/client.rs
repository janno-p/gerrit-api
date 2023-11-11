use reqwest::{Client, StatusCode, Response, Error};
use serde::{de, ser};

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
        let result = self
            .http_client
            .get(format!("{}/{}", self.base_url, url))
            .header("Cookie", format!("GerritAccount={}", &self.account_token))
            .send()
            .await;

        parse_response(result, StatusCode::OK).await
    }

    pub(crate) async fn execute<B, T>(&self, url: String, value: &B) -> Result<T, GerritError>
    where
        B: ser::Serialize,
        T: de::DeserializeOwned,
    {
        let result = self
            .http_client
            .put(format!("{}/{}", self.base_url, url))
            .header("Cookie", format!("GerritAccount={}", &self.account_token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&value)
            .send()
            .await;

        parse_response(result, StatusCode::CREATED).await
    }

    pub(crate) async fn delete<B>(&self, url: String, value: &Option<B>) -> Result<(), GerritError>
    where
        B: ser::Serialize
    {
        let builder = self
            .http_client
            .delete(format!("{}/{}", self.base_url, url))
            .header("Cookie", format!("GerritAccount={}", &self.account_token))
            .header("Content-Type", "application/json; charset=UTF-8");

        let builder = if let Some(value) = value {
            builder.json(&value)
        } else {
            builder
        };

        let result = builder.send().await;

        match result {
            Ok(response) if response.status() == StatusCode::NO_CONTENT => Ok(()),
            Ok(response) => Err(GerritError::InvalidStatusCode(response.status())),
            Err(err) => {
                err.status()
                .map(|status_code| Err(GerritError::InvalidStatusCode(status_code)))
                .unwrap_or(Err(GerritError::InvalidRequest(err)))
            },
        }
    }
}

async fn parse_response<T>(result: Result<Response, Error>, success_status_code: StatusCode) -> Result<T, GerritError>
where
    T: de::DeserializeOwned,
{
    match result {
        Ok(response) if response.status() == success_status_code => {
            if let Some(content_type) = response.headers().get("content-type") {
                let content_type = String::from_utf8(content_type.as_bytes().to_vec()).unwrap_or_default();
                if let Some("application/json") = content_type.split(";").next() {
                    let text = response.text().await;
                    match text {
                        Ok(json_string) => serde_json::from_str(&json_string[4..]).map_err(|e| { println!("{}", json_string); println!("{}", e); GerritError::InvalidJson }),
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

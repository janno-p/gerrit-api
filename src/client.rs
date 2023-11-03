use reqwest::{Client, RequestBuilder};

pub struct GerritClient {
    http_client: Client,
    base_url: String,
}

impl GerritClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
        }
    }

    pub(crate) fn get(&self, url: String) -> RequestBuilder
    {
        self.http_client.get(&format!("{}/{}", self.base_url, url))
    }
}

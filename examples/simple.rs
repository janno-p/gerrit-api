use std::env;

use dotenv::dotenv;
use gerrit_api::{documentation, GerritClient};

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let base_url = env::var("BASE_URL").expect("BASE_URL not defined");
    let api = GerritClient::new(base_url);
    let result = documentation::search("test".into()).execute(&api).await;
    println!("Documentation status: {:?}", result);
}

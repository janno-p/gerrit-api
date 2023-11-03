use std::env;

use dotenv::dotenv;
use gerrit_api::{documentation, GerritClient, projects};

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let base_url = env::var("BASE_URL").expect("BASE_URL not defined");
    let account_token = env::var("GERRIT_ACCOUNT").unwrap_or_default();

    let client = GerritClient::new(base_url, account_token);

    let result = documentation::search("test".into())
        .execute(&client)
        .await;
    println!("Response #1: {:?}", result);

    let result = projects::submit_requirements::get("Kinnistusraamat".into(), "Code-Review".into())
        .execute(&client)
        .await;
    println!("Response #2: {:?}", result);

    let result = projects::submit_requirements::list("Kinnistusraamat%2Fkris5".into())
        .execute(&client)
        .await;
    println!("Response #3: {:?}", result);

    let result = projects::project::list()
        .with_description()
        .with_limit(10)
        .execute(&client)
        .await;
    println!("Response #4: {:?}", result);
}

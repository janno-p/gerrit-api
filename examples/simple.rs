use std::env;

use dotenv::dotenv;
use gerrit_api::{documentation, GerritClient, access, projects};

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let base_url = env::var("BASE_URL").expect("BASE_URL not defined");
    let account_token = env::var("GERRIT_ACCOUNT").unwrap_or_default();

    let client = GerritClient::new(base_url, account_token);

    access_list_access_rights(&client).await;
    documentation_search(&client).await;
    projects_get_submit_requirement(&client).await;
    projects_list_submit_requirements(&client).await;
    projects_list_projects(&client).await;
}

async fn access_list_access_rights(client: &GerritClient) {
    println!("[ACCESS] LIST_ACCESS_RIGHTS");

    let result = access::list_access_rights()
        .of_project("Kinnistusraamat".into())
        .execute(&client)
        .await;

    println!("Response: {:?}", result);
}

async fn documentation_search(client: &GerritClient) {
    println!("[DOCUMENTATION] search");

    let result = documentation::search("test".into())
        .execute(&client)
        .await;

    println!("Response: {:?}", result);
}

async fn projects_get_submit_requirement(client: &GerritClient) {
    println!("[PROJECTS] GET_SUBMIT_REQUIREMENT");

    let result = projects::get_submit_requirement("Kinnistusraamat".into(), "Code-Review".into())
        .execute(&client)
        .await;

    println!("Response: {:?}", result);
}

async fn projects_list_submit_requirements(client: &GerritClient) {
    println!("[PROJECTS] LIST_SUBMIT_REQUIREMENTS");

    let result = projects::list_submit_requirements("Kinnistusraamat%2Fkris5".into())
        .execute(&client)
        .await;

    println!("Response: {:?}", result);
}

async fn projects_list_projects(client: &GerritClient) {
    println!("[PROJECTS] LIST_PROJECTS");

    let result = projects::list_projects()
        .with_description()
        .with_limit(10)
        .with_skip(30)
        .execute(&client)
        .await;

    println!("Response: {:?}", result);
}

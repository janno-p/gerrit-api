use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LabelTypeInfo {
    pub values: HashMap<String, String>,
    pub default_value: usize,
}

#[derive(Debug, Deserialize)]
pub struct WebLinkInfo {
    pub name: String,
    pub tooltip: Option<String>,
    pub url: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub id: String,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub branches: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, LabelTypeInfo>>,
    pub web_links: Option<Vec<WebLinkInfo>>,
    #[serde(rename(deserialize = "_more_projects"))]
    pub more_projects: Option<bool>,
}

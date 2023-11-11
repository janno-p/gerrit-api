use serde::Deserialize;

use crate::accounts::AccountInfo;

#[derive(Debug, Deserialize)]
pub struct GroupOptionsInfo {
    #[serde(default)]
    pub visible_to_all: bool,
}

#[derive(Debug, Deserialize)]
pub struct GroupInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub options: Option<GroupOptionsInfo>,
    pub description: Option<String>,
    pub group_id: Option<u32>,
    pub owner: Option<String>,
    pub owner_id: Option<String>,
    pub created_on: Option<String>,
    #[serde(default, rename(deserialize = "_more_groups"))]
    pub more_groups: bool,
    pub members: Option<Vec<AccountInfo>>,
    pub includes: Option<Vec<GroupInfo>>,
}

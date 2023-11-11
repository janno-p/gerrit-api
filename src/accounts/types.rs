use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AvatarInfo {
    pub url: String,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    #[serde(rename(deserialize = "_account_id"))]
    pub account_id: u64,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub secondary_emails: Option<Vec<String>>,
    pub username: Option<String>,
    pub avatars: Option<Vec<AvatarInfo>>,
    #[serde(default, rename(deserialize = "_more_accounts"))]
    pub more_accounts: bool,
    pub status: Option<String>,
    #[serde(default)]
    pub inactive: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

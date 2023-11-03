use std::collections::HashMap;

use serde::Deserialize;

use crate::{groups::GroupInfo, projects::{WebLinkInfo, ProjectInfo}};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum RuleAction {
    Allow,
    Deny,
    Block,
    Interactive,
    Batch,
}

#[derive(Debug, Deserialize)]
pub struct PermissionRuleInfo {
    pub action: RuleAction,
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub min: i32,
    #[serde(default)]
    pub max: i32,
}

#[derive(Debug, Deserialize)]
pub struct PermissionInfo {
    pub label: Option<String>,
    #[serde(default)]
    pub exclusive: bool,
    pub rules: HashMap<String, PermissionRuleInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AccessSectionInfo {
    pub permissions: HashMap<String, PermissionInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectAccessInfo {
    pub revision: String,
    pub inherits_from: Option<ProjectInfo>,
    pub local: HashMap<String, AccessSectionInfo>,
    #[serde(default)]
    pub is_owner: bool,
    pub owner_of: Vec<String>,
    #[serde(default)]
    pub can_upload: bool,
    #[serde(default)]
    pub can_add: bool,
    #[serde(default)]
    pub can_add_tags: bool,
    #[serde(default)]
    pub config_visible: bool,
    pub groups: HashMap<String, GroupInfo>,
    pub config_web_links: Option<Vec<WebLinkInfo>>,
}

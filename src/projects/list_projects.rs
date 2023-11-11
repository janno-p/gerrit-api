use std::collections::HashMap;

use crate::{client::GerritError, GerritClient};

use super::ProjectInfo;

pub enum ProjectName {
    Prefix(String),
    Regex(String),
    Substring(String),
}

pub enum ProjectStatus {
    All,
    Active,
    ReadOnly,
    Hidden,
}

pub enum ProjectType {
    All,
    Code,
    Permissions,
}

pub struct ListProjectsBuilder {
    branch: Option<String>,
    include_description: Option<bool>,
    limit: Option<u32>,
    name: Option<ProjectName>,
    skip: Option<u32>,
    tree: Option<bool>,
    r#type: Option<ProjectType>,
    status: Option<ProjectStatus>,
}

pub fn list_projects() -> ListProjectsBuilder {
    ListProjectsBuilder {
        branch: None,
        include_description: None,
        limit: None,
        name: None,
        skip: None,
        tree: None,
        r#type: None,
        status: None,
    }
}

impl ListProjectsBuilder {
    pub fn with_branch(mut self, value: String) -> Self {
        self.branch = Some(value);
        self
    }

    pub fn with_description(mut self) -> Self {
        self.include_description = Some(true);
        self
    }

    pub fn with_limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn with_name_filter(mut self, value: ProjectName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn with_skip(mut self, value: u32) -> Self {
        self.skip = Some(value);
        self
    }

    pub fn with_tree(mut self) -> Self {
        self.tree = Some(true);
        self
    }

    pub fn with_type(mut self, value: ProjectType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn with_status(mut self, value: ProjectStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub async fn execute(
        &self,
        client: &GerritClient,
    ) -> Result<HashMap<String, ProjectInfo>, GerritError> {
        let mut url = "projects/?".to_string();

        if let Some(b) = &self.branch {
            url = format!("{}b={}&", url, b);
        }

        if let Some(true) = &self.include_description {
            url = format!("{}d=&", url);
        }

        if let Some(n) = &self.limit {
            url = format!("{}n={}&", url, n);
        }

        if let Some(m) = &self.name {
            url = match m {
                ProjectName::Prefix(p) => format!("{}p={}&", url, p),
                ProjectName::Regex(r) => format!("{}r={}&", url, r),
                ProjectName::Substring(s) => format!("{}m={}&", url, s),
            }
        }

        if let Some(s) = &self.skip {
            url = format!("{}S={}&", url, s);
        }

        if let Some(true) = &self.tree {
            url = format!("{}t=&", url);
        }

        if let Some(t) = &self.r#type {
            url = match t {
                ProjectType::All => format!("{}type=ALL&", url),
                ProjectType::Code => format!("{}type=CODE&", url),
                ProjectType::Permissions => format!("{}type=PERMISSIONS&", url),
            }
        }

        if let Some(f) = &self.status {
            url = match f {
                ProjectStatus::All => format!("{}all=&", url),
                ProjectStatus::Active => format!("{}state=ACTIVE&", url),
                ProjectStatus::Hidden => format!("{}state=HIDDEN&", url),
                ProjectStatus::ReadOnly => format!("{}state=READ_ONLY&", url),
            }
        }

        url = url.trim_end_matches(&['?', '&']).to_string();

        client.query(url).await
    }
}

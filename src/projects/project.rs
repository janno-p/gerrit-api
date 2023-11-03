use std::collections::HashMap;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::{GerritClient, client::GerritError};

#[derive(Debug, Deserialize)]
pub struct LabelTypeInfo {
    pub values: IndexMap<String, String>,
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

pub enum ProjectType {
    All,
    Code,
    Permissions,
}

pub enum ProjectState {
    Active,
    ReadOnly,
    Hidden,
}

pub enum ProjectFilter {
    All,
    State(ProjectState),
}

pub enum ProjectMatch {
    Prefix(String),
    Regex(String),
    Substring(String),
}

pub struct ListProjectsBuilder {
    branch: Option<String>,
    include_description: Option<bool>,
    limit: Option<u32>,
    project_match: Option<ProjectMatch>,
    skip: Option<u32>,
    tree: Option<bool>,
    project_type: Option<ProjectType>,
    filter: Option<ProjectFilter>,
}

pub fn list() -> ListProjectsBuilder {
    ListProjectsBuilder {
        branch: None,
        include_description: None,
        limit: None,
        project_match: None,
        skip: None,
        tree: None,
        project_type: None,
        filter: None
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

    pub fn with_match(mut self, value: ProjectMatch) -> Self {
        self.project_match = Some(value);
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
        self.project_type = Some(value);
        self
    }

    pub fn with_filter(mut self, value: ProjectFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub async fn execute(&self, client: &GerritClient) -> Result<HashMap<String, ProjectInfo>, GerritError> {
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

        if let Some(m) = &self.project_match {
            url = match m {
                ProjectMatch::Prefix(p) => format!("{}p={}&", url, p),
                ProjectMatch::Regex(r) => format!("{}r={}&", url, r),
                ProjectMatch::Substring(s) => format!("{}m={}&", url, s),
            }
        }

        if let Some(s) = &self.skip {
            url = format!("{}S={}&", url, s);
        }

        if let Some(true) = &self.tree {
            url = format!("{}t=&", url);
        }

        if let Some(t) = &self.project_type {
            url = match t {
                ProjectType::All => format!("{}type=ALL&", url),
                ProjectType::Code => format!("{}type=CODE&", url),
                ProjectType::Permissions => format!("{}type=PERMISSIONS&", url),
            }
        }

        if let Some(f) = &self.filter {
            url = match f {
                ProjectFilter::All => format!("{}all=&", url),
                ProjectFilter::State(ProjectState::Active) => format!("{}state=ACTIVE&", url),
                ProjectFilter::State(ProjectState::Hidden) => format!("{}state=HIDDEN&", url),
                ProjectFilter::State(ProjectState::ReadOnly) => format!("{}state=READ_ONLY&", url),
            }
        }

        url = url.trim_end_matches(&['?', '&']).to_string();

        client.query(url).await
    }
}

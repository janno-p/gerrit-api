use std::collections::HashMap;

use crate::{client::GerritError, GerritClient};

use super::ProjectAccessInfo;

pub struct ListAccessRightsBuilder {
    projects: Vec<String>,
}

pub fn list_access_rights() -> ListAccessRightsBuilder {
    ListAccessRightsBuilder {
        projects: Vec::new(),
    }
}

impl ListAccessRightsBuilder {
    pub fn of_project(mut self, project_name: String) -> Self {
        self.projects.push(project_name);
        self
    }

    pub async fn execute(
        &self,
        client: &GerritClient,
    ) -> Result<HashMap<String, ProjectAccessInfo>, GerritError> {
        let url = self
            .projects
            .iter()
            .fold("access/?".to_string(), |acc, p| {
                format!("{}project={}&", acc, p)
            })
            .trim_end_matches(&['?', '&'])
            .to_string();

        client.query(url).await
    }
}

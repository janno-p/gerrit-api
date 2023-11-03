use serde::Deserialize;

use crate::{GerritClient, client::GerritError};

#[derive(Debug, Deserialize)]
pub struct SubmitRequirementInfo {
    pub name: String,
    pub description: Option<String>,
    pub applicability_expression: Option<String>,
    pub submittability_expression: String,
    pub override_expression: Option<String>,
    pub allow_override_in_child_projects: bool,
}

pub fn create() {

}

pub fn update() {

}

pub struct GetSubmitRequirementBuilder {
    project_name: String,
    submit_requirement_name: String,
}

pub fn get(project_name: String, submit_requirement_name: String) -> GetSubmitRequirementBuilder {
    GetSubmitRequirementBuilder { project_name, submit_requirement_name }
}

pub struct ListSubmitRequirementsBuilder {
    project_name: String,
}

pub fn list(project_name: String) -> ListSubmitRequirementsBuilder {
    ListSubmitRequirementsBuilder { project_name }
}

pub fn delete() {

}

impl GetSubmitRequirementBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<SubmitRequirementInfo, GerritError> {
        client.query(format!("projects/{}/submit_requirements/{}", self.project_name, self.submit_requirement_name)).await
    }
}

impl ListSubmitRequirementsBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<Vec<SubmitRequirementInfo>, GerritError> {
        client.query(format!("projects/{}/submit_requirements", self.project_name)).await
    }
}

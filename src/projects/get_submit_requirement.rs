use crate::{GerritClient, client::GerritError};

use super::SubmitRequirementInfo;

pub struct GetSubmitRequirementBuilder {
    project_name: String,
    submit_requirement_name: String,
}

pub fn get_submit_requirement(project_name: String, submit_requirement_name: String) -> GetSubmitRequirementBuilder {
    GetSubmitRequirementBuilder { project_name, submit_requirement_name }
}

impl GetSubmitRequirementBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<SubmitRequirementInfo, GerritError> {
        client.query(format!("projects/{}/submit_requirements/{}", self.project_name, self.submit_requirement_name)).await
    }
}
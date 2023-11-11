use serde::Serialize;

use crate::{client::GerritError, GerritClient};

use super::SubmitRequirementInfo;

#[derive(Debug, Serialize)]
struct SubmitRequirementInput {
    name: String,
    description: Option<String>,
    applicability_expression: Option<String>,
    submittability_expression: String,
    override_expression: Option<String>,
    allow_override_in_child_projects: Option<bool>,
}

pub struct CreateSubmitRequirementBuilder {
    project_name: String,
    submit_requirement_input: SubmitRequirementInput,
}

pub fn create_submit_requirement(
    project_name: String,
    submit_requirement_name: String,
) -> CreateSubmitRequirementBuilder {
    CreateSubmitRequirementBuilder {
        project_name,
        submit_requirement_input: SubmitRequirementInput {
            name: submit_requirement_name,
            description: None,
            applicability_expression: None,
            submittability_expression: String::new(),
            override_expression: None,
            allow_override_in_child_projects: None,
        },
    }
}

impl CreateSubmitRequirementBuilder {
    pub async fn execute(
        &self,
        client: &GerritClient,
    ) -> Result<SubmitRequirementInfo, GerritError> {
        client
            .execute(
                format!(
                    "projects/{}/submit_requirements/{}",
                    self.project_name, self.submit_requirement_input.name
                ),
                &self.submit_requirement_input,
            )
            .await
    }
}

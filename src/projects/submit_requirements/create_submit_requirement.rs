use crate::{client::GerritError, GerritClient};

use super::{SubmitRequirementInfo, SubmitRequirementInput};

pub struct CreateSubmitRequirementBuilder {
    project_name: String,
    submit_requirement_input: SubmitRequirementInput,
}

/// Creates a new submit requirement definition in this project.
/// The calling user must have write access to the `refs/meta/config` branch of the project.
/// If a submit requirement with this name is already defined in this project,
/// this submit requirement definition is updated (see `update_submit_requirement`).
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

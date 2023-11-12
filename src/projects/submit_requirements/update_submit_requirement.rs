use crate::{client::GerritError, GerritClient};

use super::{SubmitRequirementInfo, SubmitRequirementInput};

pub struct UpdateSubmitRequirementBuilder {
    project_name: String,
    submit_requirement_input: SubmitRequirementInput,
}

/// Updates the definition of a submit requirement that is defined in this project.
/// The calling user must have write access to the `refs/meta/config` branch of the project.
/// The new submit requirement will overwrite the existing submit requirement. That is,
/// unspecified attributes will be set to their defaults.
pub fn update_submit_requirement(
    project_name: String,
    submit_requirement_name: String,
) -> UpdateSubmitRequirementBuilder {
    UpdateSubmitRequirementBuilder {
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

impl UpdateSubmitRequirementBuilder {
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

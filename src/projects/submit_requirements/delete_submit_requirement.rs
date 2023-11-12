use crate::{client::GerritError, GerritClient};

pub struct DeleteSubmitRequirementBuilder {
    project_name: String,
    submit_requirement_name: String,
}

/// Deletes the definition of a submit requirement that is defined in this project.
/// The calling user must have write access to the `refs/meta/config` branch of the project.
/// The request would fail with `404 Not Found` if there is no submit requirement with this name
/// for this project.
pub fn delete_submit_requirement(
    project_name: String,
    submit_requirement_name: String,
) -> DeleteSubmitRequirementBuilder {
    DeleteSubmitRequirementBuilder {
        project_name,
        submit_requirement_name,
    }
}

impl DeleteSubmitRequirementBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<(), GerritError> {
        client
            .delete::<()>(
                format!(
                    "projects/{}/submit_requirements/{}",
                    self.project_name, self.submit_requirement_name
                ),
                &None,
            )
            .await
    }
}

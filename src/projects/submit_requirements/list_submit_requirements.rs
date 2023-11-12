use crate::{client::GerritError, GerritClient};

use super::SubmitRequirementInfo;

pub struct ListSubmitRequirementsBuilder {
    project_name: String,
}

/// Retrieves a list of all submit requirements for this project. The `inherited` parameter
/// can be supplied to also list submit requirements from parent projects.
/// The calling user must have read access to the `refs/meta/config` branch of the project.
/// If the `inherited` option is used, the caller must have read access to the `refs/meta/config`
/// branch of all parent projects as well.
pub fn list_submit_requirements(project_name: String) -> ListSubmitRequirementsBuilder {
    ListSubmitRequirementsBuilder { project_name }
}

impl ListSubmitRequirementsBuilder {
    pub async fn execute(
        &self,
        client: &GerritClient,
    ) -> Result<Vec<SubmitRequirementInfo>, GerritError> {
        client
            .query(format!(
                "projects/{}/submit_requirements",
                self.project_name
            ))
            .await
    }
}

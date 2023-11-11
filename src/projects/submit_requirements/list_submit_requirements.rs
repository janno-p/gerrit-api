use crate::{client::GerritError, GerritClient};

use super::SubmitRequirementInfo;

pub struct ListSubmitRequirementsBuilder {
    project_name: String,
}

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

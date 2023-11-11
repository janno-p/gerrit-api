use crate::{GerritClient, client::GerritError};

use super::DashboardInfo;

pub struct ListDashboardsBuilder {
    project_name: String,
}

/// List custom dashboards for a project.
pub fn list_dashboards(project_name: String) -> ListDashboardsBuilder {
    ListDashboardsBuilder { project_name }
}

impl ListDashboardsBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<Vec<DashboardInfo>, GerritError> {
        client.query(format!("projects/{}/dashboards/", self.project_name)).await
    }
}

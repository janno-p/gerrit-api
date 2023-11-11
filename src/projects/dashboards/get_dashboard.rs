use crate::{GerritClient, client::GerritError};

use super::DashboardInfo;

pub struct GetDashboardBuilder {
    project_name: String,
    dashboard_id: String,
}

/// Retrieves a project dashboard. The dashboard can be defined on that project or be inherited from a parent project.
pub fn get_dashboard(project_name: String, dashboard_id: String) -> GetDashboardBuilder {
    GetDashboardBuilder { project_name, dashboard_id }
}

impl GetDashboardBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<DashboardInfo, GerritError> {
        client.query(format!("projects/{}/dashboards/{}", self.project_name, self.dashboard_id)).await
    }
}

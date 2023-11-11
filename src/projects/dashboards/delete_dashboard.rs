use crate::{GerritClient, client::GerritError};

use super::DashboardInput;

pub struct DeleteDashboardBuilder {
    project_name: String,
    dashboard_id: String,
    dashboard_input: Option<DashboardInput>,
}

/// Deletes a project dashboard.
/// Currently only supported for the `default` dashboard.
pub fn delete_dashboard(project_name: String, dashboard_id: String) -> DeleteDashboardBuilder {
    DeleteDashboardBuilder { project_name, dashboard_id, dashboard_input: None }
}

impl DeleteDashboardBuilder {
    pub fn with_input(mut self, dashboard_input: DashboardInput) -> Self {
        self.dashboard_input = Some(dashboard_input);
        self
    }

    pub async fn execute(&self, client: &GerritClient) -> Result<(), GerritError> {
        client.delete(format!("projects/{}/dashboards/{}", self.project_name, self.dashboard_id), &self.dashboard_input).await
    }
}

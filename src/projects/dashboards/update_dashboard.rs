use crate::{client::GerritError, GerritClient};

use super::{DashboardInfo, DashboardInput};

pub struct UpdateDashboardBuilder {
    project_name: String,
    dashboard_id: String,
    dashboard_input: DashboardInput,
}

/// Updates a project dashboard, if a project dashboard with the given dashboard ID already exists.
/// Currently only supported for the `default` dashboard.
pub fn update_dashboard(
    project_name: String,
    dashboard_id: String,
    dashboard_input: DashboardInput,
) -> UpdateDashboardBuilder {
    UpdateDashboardBuilder {
        project_name,
        dashboard_id,
        dashboard_input,
    }
}

impl UpdateDashboardBuilder {
    pub async fn execute(&self, client: &GerritClient) -> Result<DashboardInfo, GerritError> {
        client
            .execute(
                format!(
                    "projects/{}/dashboards/{}",
                    self.project_name, self.dashboard_id
                ),
                &self.dashboard_input,
            )
            .await
    }
}

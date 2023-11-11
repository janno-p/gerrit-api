use crate::{client::GerritError, GerritClient};

use super::{DashboardInfo, DashboardInput};

pub struct CreateDashboardBuilder {
    project_name: String,
    dashboard_id: String,
    dashboard_input: DashboardInput,
}

/// Creates a project dashboard, if a project dashboard with the given dashboard ID doesn’t exist yet.
/// Currently only supported for the `default` dashboard.
pub fn create_dashboard(
    project_name: String,
    dashboard_id: String,
    dashboard_input: DashboardInput,
) -> CreateDashboardBuilder {
    CreateDashboardBuilder {
        project_name,
        dashboard_id,
        dashboard_input,
    }
}

impl CreateDashboardBuilder {
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

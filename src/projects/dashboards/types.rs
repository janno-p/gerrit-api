use serde::{Deserialize, Serialize};

/// The `DashboardInfo` entity contains information about a project dashboard.
#[derive(Debug, Deserialize)]
pub struct DashboardInfo {
    /// The ID of the dashboard. The ID has the format '<ref>:<path>',
    /// where ref and path are URL encoded.
    pub id: String,

    /// The name of the project for which this dashboard is returned.
    pub project: String,

    /// The name of the project in which this dashboard is defined.
    /// This is different from `project` if the dashboard is inherited from a parent project.
    pub defining_project: String,

    /// The name of the ref in which the dashboard is defined, without the `refs/meta/dashboards/`
    /// prefix, which is common for all dashboard refs.
    pub r#ref: String,

    /// The path of the file in which the dashboard is defined.
    pub path: String,

    /// The description of the dashboard.
    pub description: Option<String>,

    /// Subquery that applies to all sections in the dashboard.
    /// Tokens such as `${project}` are not resolved.
    pub foreach: Option<String>,

    /// The URL under which the dashboard can be opened in the Gerrit Web UI.
    /// The URL is relative to the canonical web URL.
    /// Tokens in the queries such as `${project}` are resolved.
    pub url: String,

    /// Whether this is the default dashboard of the project.
    #[serde(default)]
    pub is_default: bool,

    /// The title of the dashboard.
    pub title: Option<String>,

    /// The list of sections in the dashboard.
    #[serde(default)]
    pub sections: Vec<DashboardSectionInfo>,
}

/// The `DashboardSectionInfo` entity contains information about a section in a dashboard.
#[derive(Debug, Deserialize)]
pub struct DashboardSectionInfo {
    /// The title of the section.
    pub name: String,

    /// The query of the section.
    /// Tokens such as `${project}` are not resolved.
    pub query: String,
}

/// The `DashboardInput` entity contains information to create/update a project dashboard.
#[derive(Debug, Serialize)]
pub struct DashboardInput {
    /// URL encoded ID of a dashboard to which this dashboard should link to.
    pub id: Option<String>,

    /// Message that should be used to commit the change of the dashboard.
    pub commit_message: Option<String>,
}

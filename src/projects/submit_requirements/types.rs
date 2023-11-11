use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SubmitRequirementInfo {
    pub name: String,
    pub description: Option<String>,
    pub applicability_expression: Option<String>,
    pub submittability_expression: String,
    pub override_expression: Option<String>,
    pub allow_override_in_child_projects: bool,
}

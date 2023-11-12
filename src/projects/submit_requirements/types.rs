use serde::{Deserialize, Serialize};

/// The `SubmitRequirementInfo` entity describes a submit requirement.
#[derive(Debug, Deserialize)]
pub struct SubmitRequirementInfo {
    /// The submit requirement name.
    pub name: String,

    /// Description of the submit requirement.
    pub description: Option<String>,

    /// Query expression that can be evaluated on any change. If evaluated to true on a change,
    /// the submit requirement is then applicable for this change. If not specified,
    /// the submit requirement is applicable for all changes.
    pub applicability_expression: Option<String>,

    /// Query expression that can be evaluated on any change. If evaluated to true on a change,
    /// the submit requirement is fulfilled and not blocking change submission.
    pub submittability_expression: String,

    /// Query expression that can be evaluated on any change. If evaluated to true on a change,
    /// the submit requirement is overridden and not blocking change submission.
    pub override_expression: Option<String>,

    /// Whether this submit requirement can be overridden in child projects.
    pub allow_override_in_child_projects: bool,
}

/// The `SubmitRequirementInput` entity describes a submit requirement.
#[derive(Debug, Serialize)]
pub struct SubmitRequirementInput {
    /// The submit requirement name.
    pub name: String,

    /// Description of the submit requirement.
    pub description: Option<String>,

    /// Query expression that can be evaluated on any change. If evaluated to true on a change,
    /// the submit requirement is then applicable for this change. If not specified,
    /// the submit requirement is applicable for all changes.
    pub applicability_expression: Option<String>,

    /// Query expression that can be evaluated on any change. If evaluated to true on a change,
    /// the submit requirement is fulfilled and not blocking change submission.
    pub submittability_expression: String,

    /// Query expression that can be evaluated on any change. If evaluated to true on a change,
    /// the submit requirement is overridden and not blocking change submission.
    pub override_expression: Option<String>,

    /// Whether this submit requirement can be overridden in child projects. Default is `false`.
    pub allow_override_in_child_projects: Option<bool>,
}

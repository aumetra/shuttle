use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Schema used in `examples/templates.toml` and services that parses it
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatesSchema {
    pub templates: HashMap<String, TemplateDefinition>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TemplateDefinition {
    /// Title of the template
    pub title: String,
    /// A short description of the template
    pub description: Option<String>,
    /// Path relative to the repo root
    pub path: Option<String>,
    /// "starter" OR "template" (default) OR "tutorial"
    #[serde(default)]
    pub r#type: TemplateType,
    /// List of areas where this template is useful. Examples: "Web app", "Discord bot", "Monitoring", "Automation", "Utility"
    pub use_cases: Vec<String>,
    /// List of keywords that describe the template. Examples: "axum", "serenity", "typescript", "saas", "fullstack", "database"
    pub tags: Vec<String>,
    /// URL to a live instance of the template (if relevant)
    pub live_demo: Option<String>,

    /// If this template is available in the `cargo shuttle init --template` short-hand options, add that name here
    pub template: Option<String>,

    /// Set this to true if this is a community template outside of the shuttle-examples repo
    pub community: Option<bool>,
    /// GitHub username of the author of the community template
    pub author: Option<String>,
    /// URL to the repo of the community template
    pub repo: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateType {
    Starter,
    #[default]
    Template,
    Tutorial,
}

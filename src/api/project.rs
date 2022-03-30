use std::fmt;
use crate::clockify::Config; 
use crate::api::{
    EndPoint,
    task::Task,
    common::{Rate, Membership},
}; 
use crate::error::Error;
use crate::ui::components::Id;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Option<String>,
    pub name: String,
    pub hourly_rate: Option<Rate>,
    pub client_id: Option<String>, 
    pub client: Option<String>, 
    pub workspace_id: Option<String>,
    pub billable: Option<bool>, 
    pub memberships: Option<Vec<Membership>>, 
    pub color: Option<String>, 
    pub estimate: Option<Estimate>, 
    pub archived: Option<bool>,
    pub tasks: Option<Vec<Task>>, 
    pub note: Option<String>, 
    pub duration: Option<String>, 
    pub cost_rate: Option<String>, 
    pub time_estimate: Option<TimeEstimate>, 
    pub budget_estimate: Option<String>, 
    pub custom_fields: Option<Vec<CustomField>>, 
    pub public: Option<bool>, 
    pub template: Option<bool>, 
    pub favorite: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Estimate {
    pub estimate: String, 
    pub r#type: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeEstimate {
    pub estimate: String, 
    pub r#type: String, 
    pub reset_option: Option<String>, 
    pub active: bool, 
    pub include_non_billable: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub custom_field_id: String, 
    pub name: String, 
    pub r#type: String, 
    pub value: String, 
    pub status: String
}

impl From<&str> for Project {
    fn from(s: &str) -> Project {
        let mut default = Project::default();
        default.name = s.to_string();
        default
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Id for Project {
    fn id(&self) -> String {
        return self.id.as_ref().unwrap().clone(); 
    }
}

impl EndPoint for Project {
    fn endpoint(config: &Config) -> Result<String, Error> {
        Ok(format!("/workspaces/{}/projects", config.workspace_id.as_ref().ok_or(Error::MissingWorkspace)?.clone()))
    }
}

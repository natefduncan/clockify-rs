use std::fmt;
use crate::clockify::Config; 
use crate::api::{EndPoint, EndpointError}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Option<String>,
    pub name: String,
    pub hourly_rate: Option<HourlyRate>,
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
pub struct HourlyRate {
    pub amount: u32, 
    pub currency: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub user_id: String, 
    pub hourly_rate: Option<String>, 
    pub cost_rate: Option<String>, 
    pub target_id: String, 
    pub membership_type: String, 
    pub membership_status: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Estimate {
    pub estimate: String, 
    pub r#type: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Task {
    pub id: String, 
    pub name: String,
    pub project_id: String, 
    pub assignee_ids: Vec<String>, 
    pub assignee_id: String, 
    pub user_group_ids: Vec<String>, 
    pub estimate: String, 
    pub status: String, 
    pub duration: Option<String>, 
    pub billable: bool, 
    pub hourly_rate: Option<String>, 
    pub cost_rate: Option<String>
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

impl EndPoint for Project {
    fn endpoint(config: &Config) -> String {
        format!("/workspaces/{}/projects", config.workspace_id.as_ref().unwrap().clone())
    }
}

use reqwest::blocking::Client; 
use std::fmt;
use crate::{
    clockify::Config, 
    api::{
        EndPoint, 
        tag::Tag,
    }
};
use serde::{Serialize, Deserialize};

// On list or get for TimeEntry, the start and end will show up in
// TimeInterval.start and TimeInterval.end, not TimeEntry.start and TimeEntry.end
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub start: Option<String>, // To start the timer, send with only
    pub end: Option<String>, // To end timer, send with only end
    pub billable: Option<bool>, 
    pub is_locked: Option<bool>, 
    pub id: Option<String>,
    pub description: Option<String>,
    pub project_id: Option<String>, 
    pub task_id: Option<String>, 
    pub tag_ids: Option<Vec<Tag>>, 
    pub time_interval: Option<TimeInterval>, 
    pub user_id: Option<String>, 
    pub workspace_id: Option<String>,
    pub custom_field_values: Option<Vec<CustomFieldValue>> 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeInterval {
    pub duration : Option<String>, 
    pub end: Option<String>, 
    pub start: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldValue {
    pub custom_field_id: String,
    pub time_entry_id: String, 
    pub value: String, 
    pub name: String, 
    pub r#type: String
}

impl TimeEntry {
    pub fn start(&self, _client: &Client, _config: &Config) {
        // TODO: Implement 
    }

    pub fn end(&self, _client: &Client, _config: &Config) {
        // TODO: Implement
    }
}

impl fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(d) = &self.description {
            write!(f, "{}", d)
        } else {
            write!(f, "{}", "No Description") 
        }
    }
}

impl EndPoint for TimeEntry {
    fn endpoint(config: &Config) -> String {
        format!("/workspaces/{}/time-entries", config.workspace_id.as_ref().unwrap().clone())
    }
}

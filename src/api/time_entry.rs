use std::fmt;
use crate::{
    clockify::Config, 
    api::{
        EndPoint,
    }, 
    error::Error, 
};
use serde::{Serialize, Deserialize};
use crate::ui::components::Id;

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
    pub tag_ids: Option<Vec<String>>, 
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

impl fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(d) = &self.description {
            write!(f, "{}", d)
        } else {
            write!(f, "No Description") 
        }
    }
}

impl Id for TimeEntry {
    fn id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }
}

impl EndPoint for TimeEntry {
    fn endpoint(config: &Config) -> Result<String, Error> {
        Ok(format!("/workspaces/{}/user/{}/time-entries", config.workspace_id.as_ref().ok_or(Error::MissingWorkspace)?.clone(), config.user_id.as_ref().ok_or(Error::MissingUser)?))
    }
}

use std::fmt;
use crate::{
    clockify::Config, 
    api::{
        EndPoint, 
        common::Membership,
    }, error::Error
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub active_workspace: Option<String>, 
    pub default_workspace: Option<String>, 
    pub email: String, 
    pub id: Option<String>, 
    pub memberships: Option<Vec<Membership>>, 
    pub name: Option<String>, 
    pub profile_picture: Option<String>, 
    pub settings: Option<Settings>, 
    pub status: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub week_start: String, 
    pub time_zone: String, 
    pub time_format: String, 
    pub date_format: String, 
    pub send_newsletter: bool, 
    pub weekly_updates: bool, 
    pub long_running: bool, 
    pub scheduled_reports: bool, 
    pub approval: bool, 
    pub pto: bool, 
    pub alerts: bool, 
    pub reminders: bool, 
    pub time_tracking_manual: bool, 
    pub summary_report_settings: Option<SummaryReportSettings>, 
    pub is_compact_view_on: bool, 
    pub dashboard_selection: String, 
    pub dashboard_view_type: String, 
    pub dashboard_pin_to_top: bool, 
    pub project_list_collapse: Option<String>, 
    pub collapse_all_project_lists: bool, 
    pub group_similar_entries_disabled: bool, 
    pub my_start_of_day: String, 
    pub project_picker_task_filter: bool, 
    pub lang: Option<String>, 
    pub theme: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummaryReportSettings {
    pub group: Option<String>, 
    pub subgroup: Option<String>
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

impl EndPoint for User {
    fn endpoint(config: &Config) -> Result<String, Error> {
       Ok(format!("/workspaces/{}/users", config.workspace_id.as_ref().ok_or(Error::MissingWorkspace)?.clone()))
    }
}

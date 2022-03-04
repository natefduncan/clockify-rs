use std::fmt;
use crate::{
    clockify::Config, 
    api::{
        EndPoint, 
        common::Membership,
    }
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub active_workspace: Option<String>, 
    pub default_workspace: Option<String>, 
    pub email: String, 
    pub id: Option<String>, 
    pub membership: Option<Vec<Membership>>, 
    pub name: Option<String>, 
    pub profile_picture: Option<String>, 
    pub settings: Option<Settings>, 
    pub status: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub collapse_all_project_lists : bool, 
    pub dashboard_pin_to_top : bool, 
    pub dashboard_selection: String, 
    pub dashboard_view_type: String, 
    pub date_format: String, 
    pub is_compact_view_on: bool, 
    pub long_running: bool, 
    pub project_list_collapse: Option<String>, 
    pub send_news_letter: bool, 
    pub summary_report_settings: Option<SummaryReportSettings>, 
    pub time_format: String, 
    pub time_tracking_manual: bool, 
    pub time_zone: String, 
    pub week_start: String, 
    pub weekly_updates: bool
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
    fn endpoint(config: &Config) -> String {
        format!("/workspaces/{}/users", config.workspace_id.as_ref().unwrap().clone())
    }
}

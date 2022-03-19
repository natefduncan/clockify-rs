use std::fmt;
use crate::clockify::Config; 
use crate::api::{
    EndPoint, 
    common::{Rate, Membership}
}; 
use serde::{Serialize, Deserialize};
use crate::ui::components::Id;

// Name is the only required field to create a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub hourly_rate: Option<Rate>, 
    pub id: Option<String>, 
    pub image_url: Option<String>, 
    pub memberships: Option<Vec<Membership>>,
    pub name: String, 
    pub workspace_settings: Option<Setting>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    pub admin_only_pages: Option<Vec<String>>, 
    pub automatic_lock: Option<AutomaticLock>, 
    pub can_see_time_sheet: bool, 
    pub can_see_tracker: bool, 
    pub default_billable_projects: bool, 
    pub force_description: bool, 
    pub force_projects: bool, 
    pub force_tags: bool, 
    pub lock_time_entries: Option<String>, 
    pub only_admins_create_project: bool, 
    pub only_admins_create_tag: bool, 
    pub only_admins_create_task: bool, 
    pub only_admins_see_all_time_entries: bool, 
    pub only_admins_see_billable_rates: bool, 
    pub only_admins_see_dashboard: bool, 
    pub only_admins_see_public_projects_entries: bool, 
    pub project_favorites: bool, 
    pub project_grouping_label: Option<String>, 
    pub project_picker_special_filter: bool, 
    pub round: Option<Round>, 
    pub time_rounding_in_reports: bool, 
    pub track_time_down_to_second: bool, 
    pub is_project_public_by_default: bool, 
    pub feature_subscription_type: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticLock {
    pub change_day: String, 
    pub day_of_month: String, 
    pub first_day: String, 
    pub older_than_period: String, 
    pub older_than_value: String, 
    pub r#type: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub minutes: String, 
    pub round: String
}

impl fmt::Display for Workspace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Id for Workspace {
    fn id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }
}

impl EndPoint for Workspace {
    fn endpoint(_config: &Config) -> String {
        format!("/workspaces")
    }
}

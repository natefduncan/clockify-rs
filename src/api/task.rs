use std::fmt;
use serde::{Serialize, Deserialize}; 
use crate::{
    clockify::Config, 
    api::{
        EndPoint, 
        common::Rate
    }
};

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
    pub hourly_rate: Option<Rate>, 
    pub cost_rate: Option<Rate>
}

impl From<&str> for Task {
    fn from(s: &str) -> Task {
        let mut default = Task::default();
        default.name = s.to_string();
        default
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl EndPoint for Task {
    fn endpoint(config: &Config) -> String {
        format!("/workspaces/{}/projects/{}/tasks", 
            config.workspace_id.as_ref().unwrap().clone(),
            config.project_id.as_ref().unwrap().clone()
        )   
    }
}

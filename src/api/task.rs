use std::fmt;
use serde::{Serialize, Deserialize}; 
use crate::{
    clockify::Config, 
    api::{
        EndPoint, 
        common::Rate
    }, ui::components::Id, error::Error
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Task {
    pub id: String, 
    pub name: String,
    pub project_id: Option<String>, 
    pub assignee_ids: Option<Vec<String>>, 
    pub assignee_id: Option<String>, 
    pub user_group_ids: Option<Vec<String>>, 
    pub estimate: Option<String>, 
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

impl Id for Task {
    fn id(&self) -> String {
        return self.id.clone();
    }
}

impl EndPoint for Task {
    fn endpoint(config: &Config) -> Result<String, Error> {
        Ok(format!("/workspaces/{}/projects/{}/tasks", 
            config.workspace_id.as_ref().ok_or(Error::MissingWorkspace)?.clone(),
            config.project_id.as_ref().ok_or(Error::MissingProject)?.clone()
        ))
    }
}

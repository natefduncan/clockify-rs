use std::fmt;
use crate::{clockify::Config, error::Error}; 
use crate::api::EndPoint; 
use serde::{Serialize, Deserialize};
use crate::ui::components::Id;

// Name is the only required field to create a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Option<String>,
    pub name: String,
    pub workspace_id: Option<String>,
    pub archived: Option<bool>
}

impl From<&str> for Tag {
    fn from(s: &str) -> Tag {
       Tag {
           id : None, 
           name : s.to_string(), 
           workspace_id : None, 
           archived : None
       }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Id for Tag {
    fn id(&self) -> String {
        return self.id.as_ref().unwrap().clone(); 
    }
}

impl EndPoint for Tag {
    fn endpoint(config: &Config) -> Result<String, Error> {
        Ok(format!("/workspaces/{}/tags", config.workspace_id.as_ref().ok_or(Error::MissingWorkspace)?.clone()))
    }
}

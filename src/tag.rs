use crate::clockify::Clockify; 
use crate::endpoint::{EndPoint, EndpointError}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub workspace_id: String,
    pub archived: bool
}

impl EndPoint for Tag {
    fn endpoint(clockify: &Clockify) -> String {
        format!("/workspaces/{}/tags", clockify.workspace_id)
    }
}

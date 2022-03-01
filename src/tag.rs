use crate::clockify::Clockify; 
use crate::endpoint::{EndPoint, EndpointError}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag {
    id: String,
    name: String,
    workspace_id: String,
    archived: String
}

impl EndPoint for Tag {
    fn endpoint(clockify: &Clockify) -> String {
        format!("/workspaces/{}/tags", clockify.workspace_id)
    }
}

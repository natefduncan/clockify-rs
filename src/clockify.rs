use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: Option<String>, 
    pub workspace_id: Option<String>,
    pub project_id: Option<String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_url: String::from("https://api.clockify.me/api/v1"), 
            api_key: None, 
            workspace_id: None,
            project_id: None
        }
    }
}

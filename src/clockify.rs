pub const BASE_URL : &str = "https:/api.clockify.me/api/v1";

pub struct Clockify {
    pub client : reqwest::blocking::Client, 
    pub api_key : String,
    pub workspace_id : String
}

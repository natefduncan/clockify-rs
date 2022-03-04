use serde::{Serialize, Deserialize}; 

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    pub amount: u32, 
    pub currency: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Membership {
    pub user_id: String, 
    pub hourly_rate: Option<Rate>, 
    pub cost_rate: Option<Rate>, 
    pub target_id: String, 
    pub membership_type: String, 
    pub membership_status: String
}


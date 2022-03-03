use serde::{Serialize, Deserialize}; 

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    pub amount: u32, 
    pub currency: String, 
}


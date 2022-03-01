mod clockify;
mod endpoint; 
mod tag;

use reqwest::blocking::Client; 

use crate::endpoint::EndPoint; 
use crate::clockify::Clockify;
use crate::tag::Tag; 

fn main() {
    let c = Clockify {
        api_key : "API_KEY".to_string(), 
        workspace_id : "WORKSPACE_ID".to_string(),
        client: Client::new(),
    };
    println!("{:?}", Tag::list(None, &c)); 
}


mod clockify;
mod api; 
mod ui; 

use reqwest::blocking::Client; 
use confy::ConfyError; 
use std::io::stdin; 
use crate::clockify::Config;
use crate::api::{
    EndPoint,
    project::Project,
}; 

fn main() -> Result<(), ConfyError> {
    let client = Client::new();
    let mut cfg : Config = confy::load("clockify")?;
    // API Key
    if cfg.api_key.is_none() {
        let mut s = String::new();
        println!("Enter Clockify API Key: ");
        stdin().read_line(&mut s).expect("Unable to read input");
        cfg.api_key = Some(format!("{}", s.trim()));
    }
    // Workspace Id
    if cfg.workspace_id.is_none() {
        let mut s = String::new(); 
        println!("Enter Clockify Workspace Id: ");
        stdin().read_line(&mut s).expect("Unable to read input: "); 
        cfg.workspace_id = Some(format!("{}", s.trim())); 
    }
    println!("{:?}", Project::list(&client, &cfg, None)); 
    confy::store("clockify", cfg)?; 
    Ok(())
}


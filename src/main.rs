mod clockify;
mod endpoint; 
mod project; 
mod tag;

use reqwest::blocking::Client; 
use confy::{load, store, ConfyError};
use std::io::{stdin, stdout, Write}; 
use crate::endpoint::EndPoint; 
use crate::clockify::Config;
use crate::tag::Tag; 
use crate::project::Project; 

fn main() -> Result<(), ConfyError> {
    let client = Client::new();
    let mut cfg : Config = confy::load("clockify")?;
    println!("{:?}", cfg); 
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


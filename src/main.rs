mod clockify;
mod api; 
mod ui; 
mod error; 

use reqwest::blocking::Client; 
use std::io::stdin; 
use crate::error::Error;
use tui::backend::CrosstermBackend; 
use crate::clockify::{App};
use crate::ui::crossterm; 
use crate::api::{
    EndPoint,
    project::Project,
}; 
use std::time::Duration; 

fn main() -> Result<(), Error> {
    let mut app = App::new("clockify-cli");
    let tick_rate = Duration::from_millis(250); 
    crossterm::run(&mut app, tick_rate).unwrap(); 
    confy::store("clockify", app.config)?;
    Ok(())
}


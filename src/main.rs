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
use clap::Parser;
use std::time::Duration; 

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    api_key: Option<String> 
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut app = App::new("clockify-cli");
    if app.config.api_key.is_none() && args.api_key.is_none() {
        return Err(Error::MissingApiKey);
    } else {
        if app.config.api_key.is_none() {
            app.config.api_key = args.api_key.clone();
        }
        let tick_rate = Duration::from_millis(250); 
        crossterm::run(&mut app, tick_rate).unwrap(); 
        confy::store("clockify", app.config)?;
        return Ok(());
    }
}


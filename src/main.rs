mod clockify;
mod api; 
mod ui; 
mod error; 

use crate::error::Error;
use crate::clockify::App;
use crate::ui::run;
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
    let mut app = App::new("Clockify")?;
    if app.config.api_key.is_none() && args.api_key.is_none() {
        Err(Error::MissingApiKey)
    } else {
        if app.config.api_key.is_none() {
            app.config.api_key = args.api_key;
        }
        let tick_rate = Duration::from_millis(150); 
        run(&mut app, tick_rate)?; 
        if let Some(e) = app.error {
            println!("{:?}", e);
        }
        confy::store("clockify", app.config)?;
        Ok(())
    }
}


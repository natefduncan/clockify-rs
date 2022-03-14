use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, 
    execute, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
}; 
use std::{
    error::Error, 
    io, 
    time::{Duration, Instant}
}; 
use tui::{
    backend::{Backend, CrosstermBackend}, 
    Terminal, 
}; 
use reqwest::blocking::Client; 
use crate::ui::screens::{draw, Screen}; 
use crate::clockify::App;

use super::components::Component; 

pub fn run(app: &mut App, tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?; 
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?; 

    // create app and run it
    let client = Client::new(); 
    let res = run_app(&mut terminal, &client, app, tick_rate); 

    // restore terminal
    disable_raw_mode()?; 
    execute!(
        terminal.backend_mut(), 
        LeaveAlternateScreen, 
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, client: &Client, app: &mut App, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw(f, client, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.current_screen {
                    Screen::WorkspaceSelection => app.workspaces.key_event(key.code), 
                    Screen::TimeEntryList => app.time_entries.key_event(key.code), 
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }   
    }
  
}

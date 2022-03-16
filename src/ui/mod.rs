pub mod components; 
pub mod screen;

use crossterm::{
    event::{self, KeyCode, KeyEvent, EnableMouseCapture, Event, DisableMouseCapture}, 
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
    layout::{Layout, Constraint},
    widgets::Paragraph, 
    Frame,
}; 
use reqwest::blocking::Client; 
use crate::{
    clockify::App, 
    api::{
        EndPoint,
        time_entry::TimeEntry, 
        workspace::Workspace
    },
    ui::components::{Component, StatefulList},
}; 

#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    WorkspaceSelection, 
    TimeEntrySelection, 
    ProjectSelection,
}

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
        terminal.draw(|f| {
            match app.current_screen {
                Screen::Home => screen::home(f, client, app, None),
                Screen::WorkspaceSelection => screen::workspace_selection(f, client, app, None),
                Screen::TimeEntrySelection => screen::time_entry_selection(f, client, app, None),
                Screen::ProjectSelection => screen::project_selection(f, client, app, None),
               _ => {}
            }
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Screen specific key event
                terminal.draw(|f| {
                    match app.current_screen {
                        Screen::WorkspaceSelection => screen::workspace_selection(f, client, app, Some(key)), 
                        Screen::TimeEntrySelection => screen::time_entry_selection(f, client, app, Some(key)), 
                        Screen::ProjectSelection => screen::project_selection(f, client, app, Some(key)),
                        _ => {}
                    }
                })?; 
                // App key events
                app.key_event(key)
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }   
    }
  
}


pub mod components; 
pub mod screen;

use crossterm::{
    event::{self, EnableMouseCapture, Event, DisableMouseCapture}, 
    execute, 
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
}; 
use std::{
    io, 
    time::{Duration, Instant}
}; 
use tui::{
    backend::{Backend, CrosstermBackend}, 
    Terminal,
}; 
use reqwest::blocking::Client; 
use crate::{
    clockify::App,
    // ui::components::Component, 
    error::Error
};

 

#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    WorkspaceSelection, 
    TimeEntrySelection, 
    ProjectSelection,
    TaskSelection,
    TagSelection,
    DescriptionEdit, 
}

pub fn run(app: &mut App, tick_rate: Duration) -> Result<(), Error> {
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
    res
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, client: &Client, app: &mut App, tick_rate: Duration) -> Result<(), Error> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| {
            let res = match app.current_screen {
                Screen::Home => screen::home(f, client, app, None),
                Screen::WorkspaceSelection => screen::workspace_selection(f, client, app, None),
                Screen::TimeEntrySelection => screen::time_entry_selection(f, client, app, None),
                Screen::ProjectSelection => screen::project_selection(f, client, app, None),
                Screen::TaskSelection => screen::task_selection(f, client, app, None),
                Screen::TagSelection => screen::tag_selection(f, client, app, None), 
                Screen::DescriptionEdit => screen::description_input(f, client, app, None), 
            };
            if let Err(e) = res {
                app.error = Some(e);
                app.should_quit = true;
            }
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Screen specific key event
                terminal.draw(|f| {
                   let res : Result<(), Error> = match app.current_screen {
                        Screen::WorkspaceSelection => screen::workspace_selection(f, client, app, Some(key)), 
                        Screen::TimeEntrySelection => screen::time_entry_selection(f, client, app, Some(key)), 
                        Screen::ProjectSelection => screen::project_selection(f, client, app, Some(key)),
                        Screen::TaskSelection => screen::task_selection(f, client, app, Some(key)),
                        Screen::TagSelection => screen::tag_selection(f, client, app, Some(key)),
                        Screen::DescriptionEdit => screen::description_input(f, client, app, Some(key)), 
                        _ => Ok(())
                    };
                   if let Err(e) = res {
                       app.error = Some(e);
                       app.should_quit = true;
                   }
                })?; 
                // App key events
                app.key_event(key, client)?
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


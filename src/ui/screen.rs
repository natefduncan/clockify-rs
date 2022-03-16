use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers}
};
use tui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph
};
use reqwest::blocking::Client;
use crate::{
    clockify::App,
    api::{
        EndPoint,
        user::User, 
        time_entry::TimeEntry, 
        workspace::Workspace, project::Project, tag::Tag
    }, 
    ui::{
        components::{StatefulList, Component}, 
        Screen
    } 
};


// Template chunks
pub fn template_screen<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App) -> Vec<Rect> {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    return chunks; 
    
}

// Home
pub fn home<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.title), chunks[0]);
    // If no user_id, send request
    if app.config.user_id.is_none() {
        let current_user = client.get(format!("{}{}", app.config.base_url, "/user"))
            .header("X-API-KEY", app.config.api_key.as_ref().unwrap().clone())
            .send().unwrap()
            .json::<User>().unwrap();
        app.config.user_id = current_user.id.clone();
    }

    // Force workspace selection
    if app.config.workspace_id.is_none() {
        app.current_screen = Screen::WorkspaceSelection;
    }
}

// Workspace selection
pub fn workspace_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.title), chunks[0]); 
    if app.workspaces.items.len() == 0 {
            app.workspaces = StatefulList::with_items(Workspace::list(client, &app.config, None).unwrap(), String::from("Select a workspace: "));
    }
    app.workspaces.render(f, chunks[1]);
    
    // Key Event
    if let Some(event) = key {
        app.workspaces.key_event(event);
        match event.code {
            KeyCode::Enter => {
                app.config.workspace_id = app.workspaces.get_selected_item().id.clone();
            }, 
            _ => {}
        }
    }
}

// Time Entry Selection
pub fn time_entry_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.title), chunks[0]); 
    if app.time_entries.items.len() == 0 {
        app.time_entries = StatefulList::with_items(TimeEntry::list(client, &app.config, None).unwrap(), String::from("Select a time entry: "));
    }
    app.time_entries.render(f, chunks[1]);
}

// Project Selection
pub fn project_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.title), chunks[0]);
    if app.projects.items.len() == 0 {
        app.projects = StatefulList::with_items(Project::list(client, &app.config, None).unwrap(), String::from("Select a project: "));
    }
    app.projects.render(f, chunks[1]);

    // Key Event
    if let Some(event) = key {
        app.projects.key_event(event);
        match event.code {
            _ => {}
        }
    }
}

// Tag Selection
pub fn tag_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.title), chunks[0]);
    if app.tags.items.len() == 0 {
        app.tags = StatefulList::with_items(Tag::list(client, &app.config, None).unwrap(), String::from("Select a tag: "));
    }
    app.tags.render(f, chunks[1]);

    // Key Event
    if let Some(event) = key {
        app.tags.key_event(event);
        match event.code {
            _ => {}
        }
    }
}


use tui::{
    backend::Backend, 
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Color, Modifier, Style}, 
    symbols,
    text::{Span, Spans}, 
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Tabs, Wrap, Widget, StatefulWidget}, 
    Frame
}; 

use reqwest::blocking::Client; 
use crate::{
    clockify::App,
    api::{EndPoint, time_entry::TimeEntry}, 
    ui::components::{StatefulList, InputBox, Component}
};

use crate::api::workspace::Workspace; 

#[derive(Debug, Clone)]
pub enum Screen {
    Home,
    WorkspaceSelection, 
    TimeEntryList
}

pub fn draw<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    f.render_widget(Paragraph::new(app.title), chunks[0]); 
    if app.config.workspace_id.is_none() {
        app.workspaces = StatefulList::with_items(Workspace::list(client, &app.config, None).unwrap());
        app.current_screen = Screen::WorkspaceSelection; 
        app.workspaces.render(f, client, chunks[1]); 
    } else {
        app.time_entries = StatefulList::with_items(TimeEntry::list(client, &app.config, None).unwrap());
        app.current_screen = Screen::TimeEntryList;
        app.time_entries.render(f, client, chunks[1]); 
    }
}

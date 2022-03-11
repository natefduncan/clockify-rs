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
    api::EndPoint, 
    ui::components::{StatefulList, InputBox}
};

use crate::api::workspace::Workspace; 

pub fn draw<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    f.render_widget(Paragraph::new(app.title), chunks[0]); 
    if app.config.api_key.is_none() {
        draw_get_api_key(f, app, chunks[1]); 
    } else if app.config.workspace_id.is_none() {
        draw_get_workspace_id(f, client, app, chunks[1]);
    } else {
        draw_time_entries(f, app, chunks[1]);
    }
}

pub fn draw_get_workspace_id<B>(f: &mut Frame<B>, client: &Client, app: &mut App, area: Rect)
    where B: Backend
{
    // If app.workspaces is zero, populate.
    if app.workspaces.items.len() == 0 {
        app.workspaces = StatefulList::with_items(Workspace::list(client, &app.config, None).unwrap());
    }
   let workspaces: Vec<ListItem> = app.workspaces
       .items
       .iter()
       .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.name.clone()))]))
       .collect();
    let workspaces = List::new(workspaces)
        .block(Block::default().borders(Borders::ALL).title("Workspaces"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(workspaces, area, &mut app.workspaces.state)
}


pub fn draw_get_api_key<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where B: Backend
{
    let block = Block::default()
        .title("Enter Clockify API Key")
        .borders(Borders::ALL); 

}


pub fn draw_time_entries<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where B: Backend
{
    let time_entries: Vec<ListItem> = app.time_entries
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.description.clone().unwrap()))]))
        // FIXME: Time Entry clone
        .collect();
    let time_entries = List::new(time_entries)
        .block(Block::default().borders(Borders::ALL).title("Time Entries"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(time_entries, area, &mut app.time_entries.state)
} 

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style}, 
    symbols,
    text::{Span, Spans}, 
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph, Row, Sparkline, Table, Tabs, Wrap
    }, 
    Frame,
};

use crate::ui::app::App; 

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    f.render_widget(Paragraph::new(app.title), chunks[0]); 
    draw_time_entries(f, app, chunks[1]);
}

pub fn draw_time_entries<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
    where B: Backend
{
    let time_entries: Vec<ListItem> = app.time_entries
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
        .collect();
    let time_entries = List::new(time_entries)
        .block(Block::default().borders(Borders::ALL).title("Time Entries"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(time_entries, area, &mut app.time_entries.state)
}   

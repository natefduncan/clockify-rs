use tui::{
    backend::Backend, 
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Color, Modifier, Style}, 
    symbols, 
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Widget, StatefulWidget, ListState},
    Frame
}; 

use std::fmt::Display;

use crossterm::event::KeyCode;
use crate::{
    clockify::App,
    api::EndPoint
};
use reqwest::blocking::Client;

pub trait Component {
   fn render<B: Backend>(&mut self, f: &mut Frame<B>, client: &Client, area: Rect);
   fn key_event(&mut self, key: KeyCode);
}

#[derive(Debug, Clone)]
pub struct InputBox {
    pub prompt: String,
    pub text: String, 
}

impl Component for InputBox {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, client: &Client, area: Rect) {
        let block = Block::default()
            .title(self.prompt.clone())
            .borders(Borders::ALL);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(5)
            .constraints(
                [
                Constraint::Percentage(25), 
                Constraint::Percentage(75)
                ].as_ref()
            ).split(area); 
        f.render_widget(Paragraph::new(self.prompt.clone()), chunks[0]); 
        f.render_widget(Paragraph::new(self.text.clone()), chunks[1]); 
    }

    fn key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.text = format!("{}{}", self.text, c); 
            },
            _ => {}
        }
    }
}

impl From<&str> for InputBox {
    fn from(s: &str) -> InputBox {
        InputBox {
            prompt: s.to_owned(), 
            text: String::new()
        }
    }
}

impl From<String> for InputBox {
    fn from(s: String) -> InputBox {
        InputBox {
            prompt: s, 
            text: String::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatefulList<T: Display> {
    pub state: ListState, 
    pub items: Vec<T>
}

impl<T: Display> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(), 
            items
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0, 
        }; 
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1 
                } else {
                    i - 1
                }
            }
            None => 0, 
        }; 
        self.state.select(Some(i))
    }
}

impl<T: Display> Component for StatefulList<T> {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, client: &Client, area: Rect) {
        let list_item : Vec<ListItem> = self.items.iter()
           .map(|i| ListItem::new(vec![Spans::from(Span::raw(format!("{}", i)))]))
           .collect();
        let list_item = List::new(list_item)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_stateful_widget(list_item, area, &mut self.state)
    }

    fn key_event(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Up => {
                self.previous()
            }, 
            KeyCode::Down => {
                self.next()
            }, 
            _ => {}
        }
        
    }
}


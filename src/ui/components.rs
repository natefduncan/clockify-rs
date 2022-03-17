use tui::{
    backend::Backend, 
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Modifier, Style}, 
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Frame
}; 
use std::fmt::Display;
use crossterm::event::{KeyCode, KeyEvent};
pub trait Component {
   fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
   fn key_event(&mut self, key: KeyEvent);
}

#[derive(Debug, Clone)]
pub struct InputBox {
    pub prompt: String,
    pub text: String, 
}

impl Component for InputBox {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints(
                [
                Constraint::Min(1), 
                Constraint::Min(0),
                ].as_ref()
            ).split(area); 
        f.render_widget(Paragraph::new(self.prompt.clone()), chunks[0]); 
        f.render_widget(Paragraph::new(self.text.clone()), chunks[1]); 
    }

    fn key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.text = format!("{}{}", self.text, c); 
            },
            KeyCode::Backspace => {
                self.text.pop();
            }
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
    pub selected: Vec<usize>, 
    pub multiselect: bool, 
    pub title: String, 
    pub state: ListState, 
    pub items: Vec<T>
}

impl<T: Display> StatefulList<T> {
    pub fn with_items(items: Vec<T>, title: String) -> StatefulList<T> {
        StatefulList {
            selected: vec![],
            multiselect: false, 
            state: ListState::default(), 
            items,
            title
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

    pub fn get_selected_item(&self) -> Option<&T> {
        let i = match self.state.selected() {
            Some(i) => i, 
            None => 0
        };
        return self.items.get(i);
    }
}

impl<T: Display> Component for StatefulList<T> {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints(
                [
                Constraint::Min(1), 
                Constraint::Min(0),
                ].as_ref()
            ).split(area); 
        f.render_widget(Paragraph::new(self.title.clone()), chunks[0]); 
        let list_item : Vec<ListItem> = self.items.iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(format!("{}", i)))]))
            .collect();
        let list_item = List::new(list_item)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_stateful_widget(list_item, chunks[1], &mut self.state)
    }

    fn key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    'j' => self.next(),
                    'k' => self.previous(),
                    _ => {}
                }
            },
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


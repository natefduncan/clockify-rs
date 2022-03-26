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
use aho_corasick::AhoCorasickBuilder;

use crate::clockify::AppMode;

pub trait Component {
   fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
   fn key_event(&mut self, key: KeyEvent, mode: &AppMode);
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

    fn key_event(&mut self, key: KeyEvent, mode: &AppMode) {
        match mode {
            AppMode::Edit => {
                match key.code {
                    KeyCode::Char(c) => {
                        self.text = format!("{}{}", self.text, c); 
                    },
                    KeyCode::Backspace => {
                        self.text.pop();
                    }, 
                    _ => {}
                }
            },
            AppMode::Navigation => {
                match key.code {
                    KeyCode::Char(c) => {
                        match c {
                            'c' => { self.text = String::new() }, 
                            _ => {}
                        }
                    }, 
                    _ => {}
                }
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
pub struct StatefulList<T: Display + Id + Clone> {
    pub search_text: String,
    pub selected: Vec<String>, 
    pub multiselect: bool, 
    pub title: String, 
    pub state: ListState, 
    pub items: Vec<T>
}

pub trait Id {
    fn id(&self) -> String; 
}

#[derive(Debug, Copy, Clone)]
struct Match<T> {
    obj: T,
    count: usize
}

impl<T: Display + Id + Clone> StatefulList<T> {
    pub fn with_items(items: Vec<T>, title: String, multiselect: bool) -> StatefulList<T> {
        StatefulList {
            search_text: String::new(),
            selected: vec![],
            multiselect, 
            state: ListState::default(), 
            items,
            title
        }
    }

    pub fn search(&self, query: &str) -> Vec<T> {
        let vec_string: Vec<String> = self.items
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let matches = Self::find_matches(query.to_owned(), &vec_string);
        let filtered = Self::filter_by_matches(&self.items, &matches);
        return filtered;
    }

    fn find_matches(query: String, vector: &Vec<String>) -> Vec<usize> {
        let patterns = query.split_whitespace();
        let ac = AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .build(patterns);
        vector
            .iter()
            .map(|string| {
                let matches = ac
                    .find_iter(string)
                    .map(|m| m.pattern())
                    .collect::<Vec<usize>>();
                matches.len()
            }).collect::<Vec<usize>>()
    }

    fn filter_by_matches(filter_vec: &Vec<T>, match_vec: &Vec<usize>) -> Vec<T> {
        let zip = filter_vec.iter().zip(match_vec.iter());
        let mut matches: Vec<Match<T>> = zip
            .map(|(x,m)| Match {
                obj: x.clone(), 
                count: *m,
            }).collect::<Vec<Match<T>>>();
        matches.sort_by(|a, b| b.count.cmp(&a.count));
        return matches
            .iter()
            .filter(|m| m.count > 0)
            .map(|m| m.obj.clone())
            .collect::<Vec<T>>();
    }

    pub fn clear_selected(&mut self) {
        self.selected = vec![];
    }

    pub fn get_by_id(&self, id: String) -> Option<&T> {
        return self.items.iter().find(|x| x.id() == id);
    }

    pub fn get_by_string(&self, string: String) -> Option<&T> {
        return self.items.iter().find(|x| x.to_string() == string);
    }

    pub fn toggle_highlighted(&mut self) {
        let highlighted_item : Option<T> = self.get_highlighted_item().cloned();
        if let Some(item) = highlighted_item.clone() {
            if let Some(idx) = self.selected.iter().position(|x| *x == item.id()) {
                self.selected.remove(idx);
            } else {
                if !self.multiselect && self.selected.len() > 0 {
                    self.selected = vec![]; 
                }
                self.selected.push(item.id()); 
            }
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

    pub fn get_highlighted_item(&self) -> Option<&T> {
        match self.state.selected() {
            Some(x) => {
                let mut items = vec![];
                if self.search_text.is_empty() {
                    items = self.items.iter().map(|x| x.clone()).collect(); // FIXME
                } else {
                    items = self.search(&self.search_text);
                }
                let highlighted_item = items.get(x).unwrap();
                return self.items.iter().find(|x| x.id() == highlighted_item.id());
            }, 
            None => {
                None
            },
        }
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        return self.items.iter().filter(|x| self.selected.contains(&x.id())).collect::<Vec<&T>>().get(0).cloned();
    }

    pub fn get_selected_items(&self) -> Vec<&T> {
        return self.items.iter().filter(|x| self.selected.contains(&x.id())).collect::<Vec<&T>>();
    }
}

impl<T: Display + Id + Clone> Component for StatefulList<T> {
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
        let mut title = self.title.clone(); 
        if !self.search_text.is_empty() {
            title = format!("{}{}", title, self.search_text);
        }
        f.render_widget(Paragraph::new(title), chunks[0]); 
        let mut items = vec![];
        if self.search_text.is_empty() {
            items = self.items.iter().map(|x| x.clone()).collect(); // FIXME
        } else {
            items = self.search(&self.search_text);
        }
        let list_item : Vec<ListItem> = items.iter()
            .map(|i| {
                if self.selected.contains(&i.id()) {
                    ListItem::new(vec![Spans::from(Span::raw(format!(">> {}", i)))])
                } else {
                    ListItem::new(vec![Spans::from(Span::raw(format!("{}", i)))])
                }
            })
            .collect();
        let list_item = List::new(list_item)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED).add_modifier(Modifier::ITALIC));
        f.render_stateful_widget(list_item, chunks[1], &mut self.state)
    }

    fn key_event(&mut self, key: KeyEvent, mode: &AppMode) {
        match mode {
            AppMode::Search => {
                self.state.select(Some(0));
                match key.code {
                    KeyCode::Char(c) => {
                        self.search_text = format!("{}{}", self.search_text, c);
                    }, 
                    KeyCode::Backspace => {
                        self.search_text.pop();
                    }, 
                   _ => {}
                }
            },
            AppMode::Navigation => {
                match key.code {
                    KeyCode::Char(c) => {
                        match c {
                            'j' => self.next(),
                            'k' => self.previous(),
                            'c' => self.clear_selected(),
                            _ => {}
                        }
                    },
                    KeyCode::Up => {
                        self.previous()
                    }, 
                    KeyCode::Down => {
                        self.next()
                    }, 
                    KeyCode::Enter => {
                        self.toggle_highlighted();
                        self.search_text = String::new();
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
}


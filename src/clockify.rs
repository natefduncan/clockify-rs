use crate::{
    api::{
        tag::Tag, 
        project::Project, 
        time_entry::TimeEntry, 
        workspace::Workspace,
    },
    ui::{
        components::{StatefulList, InputBox},
        Screen
    }
};

use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use serde::{Serialize, Deserialize};
use std::fmt; 

#[derive(Debug, Clone)]
pub enum AppMode {
    Navigation, 
    Edit
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_navigation() {
            write!(f, "{}", "Navigation")
        } else {
            write!(f, "{}", "Edit")
        }
    }
}

impl AppMode {
    pub fn is_navigation(&self) -> bool {
        return matches!(&self, AppMode::Navigation);
    }
    
    pub fn is_edit(&self) -> bool {
        return matches!(&self, AppMode::Edit);
    }
}

#[derive(Debug, Clone)]
pub struct App<'a> {
    pub title: &'a str, 
    pub should_quit: bool,
    pub config: Config,
    pub current_screen: Screen, 
    pub current_entry_id: Option<String>, 
    pub current_mode: AppMode, 
    pub workspaces: StatefulList<Workspace>,
    pub projects: StatefulList<Project>,
    pub tags: StatefulList<Tag>, 
    pub description: InputBox, 
    pub time_entries: StatefulList<TimeEntry>,
}

impl<'a> fmt::Display for App<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} Mode)", self.title, self.current_mode)

    }
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title, 
            should_quit: false, 
            config: confy::load("clockify").unwrap(), 
            current_screen: Screen::Home, 
            current_entry_id: None, 
            current_mode: AppMode::Navigation, 
            workspaces: StatefulList::with_items(vec![], String::from("Select a workspace: ")), 
            projects: StatefulList::with_items(vec![], String::from("Select a project: ")),
            tags: StatefulList::with_items(vec![], String::from("Select a tag: ")), 
            description: InputBox::from("Edit the time entry description: "), 
            time_entries: StatefulList::with_items(vec![], String::from("Select a time entry: ")), 
        }
    }

    pub fn key_event(&mut self, key: KeyEvent) {
        match key.modifiers {
            KeyModifiers::CONTROL => {
               match key.code {
                    KeyCode::Char(c) => {
                        match c {
                            'q' => { self.should_quit = true; },
                            _ => {}
                        }
                    }, 
                    _ => {},
               }
            },
            KeyModifiers::NONE => {
                if self.current_mode.is_navigation() {
                    match key.code {
                        KeyCode::Char(c) => {
                            match c {
                                'w' => { self.current_screen = Screen::WorkspaceSelection }, 
                                'e' => { self.current_screen = Screen::TimeEntrySelection },
                                'p' => { self.current_screen = Screen::ProjectSelection },
                                't' => { self.current_screen = Screen::TagSelection },
                                'd' => { self.current_screen = Screen::DescriptionEdit }, 
                                'h' => { self.current_screen = Screen::Home },
                                'i' => { self.current_mode = AppMode::Edit }, 
                                _ => {}
                            } 
                        }, 
                        _ => {}
                    } 
                } else {
                    match key.code {
                        KeyCode::Esc => {
                            self.current_mode = AppMode::Navigation;
                        }, 
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: Option<String>, 
    pub workspace_id: Option<String>,
    pub user_id: Option<String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_url: String::from("https://api.clockify.me/api/v1"), 
            api_key: None, 
            workspace_id: None,
            user_id: None
        }
    }
}

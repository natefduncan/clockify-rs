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

#[derive(Debug, Clone)]
pub struct App<'a> {
    pub title: &'a str, 
    pub should_quit: bool,
    pub config: Config,
    pub current_screen: Screen, 
    pub current_entry: Option<TimeEntry>, 
    pub workspaces: StatefulList<Workspace>,
    pub projects: StatefulList<Project>,
    pub tags: StatefulList<Tag>, 
    pub description: InputBox, 
    pub time_entries: StatefulList<TimeEntry>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title, 
            should_quit: false, 
            config: confy::load("clockify").unwrap(), 
            current_screen: Screen::Home, 
            current_entry: None, 
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
                match key.code {
                    KeyCode::Char(c) => {
                        match c {
                            'w' => { self.current_screen = Screen::WorkspaceSelection }, 
                            'e' => { self.current_screen = Screen::TimeEntrySelection },
                            'p' => { self.current_screen = Screen::ProjectSelection },
                            't' => { self.current_screen = Screen::TagSelection },
                            'd' => { self.current_screen = Screen::DescriptionEdit }, 
                            _ => {}
                        }
                    }, 
                    _ => {}
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

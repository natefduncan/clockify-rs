use crate::{
    api::{
        EndPoint,
        tag::Tag, 
        project::Project, 
        time_entry::TimeEntry, 
        workspace::Workspace, task::Task,
    },
    ui::{
        components::{StatefulList, InputBox, Id},
        Screen
    }
};

use chrono::prelude::*;
use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use std::fmt; 

#[derive(Debug, Clone)]
pub enum AppMode {
    Navigation, 
    Edit, 
    Search,
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppMode::Navigation => {
                write!(f, "{}", "Navigation")
            },
            AppMode::Edit => {
                write!(f, "{}", "Edit")
            }, 
            AppMode::Search => {
                write!(f, "{}", "Search")
            }
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

    pub fn is_search(&self) -> bool {
        return matches!(&self, AppMode::Search);
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
    pub tasks: StatefulList<Task>, 
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
            workspaces: StatefulList::with_items(vec![], String::from("Select a workspace: "), false), 
            tasks: StatefulList::with_items(vec![], String::from("Select a task: "), false), 
            projects: StatefulList::with_items(vec![], String::from("Select a project: "), false),
            tags: StatefulList::with_items(vec![], String::from("Select a tag: "), true), 
            description: InputBox::from("Edit the time entry description: "), 
            time_entries: StatefulList::with_items(vec![], String::from("Select a time entry: "), false), 
        }
    }

    pub fn get_current_entry(&mut self, client: &Client) -> Option<TimeEntry> {
        if let Some(time_entry_id) = self.current_entry_id.clone() {
            return client.get(format!("{}/workspaces/{}/time-entries/{}", self.config.base_url, self.config.workspace_id.as_ref().unwrap().clone(), time_entry_id))
                .header("X-API-KEY", self.config.api_key.as_ref().unwrap().clone())
                .send().unwrap()
                .json::<TimeEntry>().ok();
        } else {
            return None;
        }
    }

    pub fn get_current_entry_with_selections(&mut self, client: &Client) -> TimeEntry {
        let mut time_entry : TimeEntry; 
        if let Some(t) = &self.get_current_entry(client) {
            time_entry = t.clone();
        } else {
            time_entry = TimeEntry::default();
        }
        // Project
        if let Some(project) = &self.projects.get_selected_item() {
            time_entry.project_id = Some(project.id());
        }
        // Task
        if let Some(task) = &self.tasks.get_selected_item() {
            time_entry.task_id = Some(task.id());
        }
        // Tags
        time_entry.tag_ids = Some(self.tags.get_selected_items().iter().map(|tag| tag.id()).collect::<Vec<String>>());
        // Description
        time_entry.description = Some(self.description.text.clone());
        return time_entry.clone();
    }

    pub fn current_formatted_time(&self) -> String {
        let utc: DateTime<Utc> = Utc::now();
        format!("{}", utc.format("%Y-%m-%dT%H:%M:%S.000Z"))
    }

    pub fn start_entry(&mut self, client: &Client) {
        // Send POST new time entry with only start
        let mut time_entry = self.get_current_entry_with_selections(client);
        // Replace start and end times
        time_entry.id = None;
        time_entry.end = None;
        time_entry.time_interval = None;
        // Add current start time
        time_entry.start = Some(self.current_formatted_time());
        
        // POST request to create
        let time_entry = TimeEntry::create(time_entry, client, &self.config, None).unwrap();
        self.current_entry_id = time_entry.id;
    }

    pub fn stop_entry(&mut self, client: &Client) {
        // Send PATCH with only end
        let mut time_entry = TimeEntry::default(); 
        time_entry.end = Some(self.current_formatted_time());
        TimeEntry::patch(time_entry, client, &self.config, None).unwrap();
    }

    pub fn update_entry(&mut self, client: &Client) {
        // Send PATCH with only end
        let time_entry = self.get_current_entry_with_selections(client);
        // POST request to create
        TimeEntry::update(time_entry, client, &self.config, None).unwrap();
    }

    pub fn key_event(&mut self, key: KeyEvent, client: &Client) {
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
                                't' => { self.current_screen = Screen::TimeEntrySelection },
                                'p' => { self.current_screen = Screen::ProjectSelection },
                                'g' => { self.current_screen = Screen::TagSelection },
                                'y' => { 
                                    self.current_screen = Screen::TaskSelection;
                                    // If selected project has changed, clear tasks
                                    if let Some(config_project_id) = &self.config.project_id {
                                        if let Some(selected_project) = self.projects.get_selected_item() {
                                            if config_project_id.clone() != selected_project.clone().id() {
                                                self.tasks.items = vec![];
                                            }
                                        }
                                    }
                                },
                                'd' => { self.current_screen = Screen::DescriptionEdit }, 
                                'h' => { self.current_screen = Screen::Home },
                                'i' => { self.current_mode = AppMode::Edit }, 
                                '/' => { self.current_mode = AppMode::Search },
                                'u' => { self.update_entry(client) },
                                's' => { self.start_entry(client) }, 
                                'e' => { self.stop_entry(client) },
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
    pub project_id: Option<String>, 
    pub user_id: Option<String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_url: String::from("https://api.clockify.me/api/v1"), 
            api_key: None, 
            workspace_id: None,
            project_id: None,
            user_id: None
        }
    }
}

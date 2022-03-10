use crate::{
    api::{
        tag::Tag, 
        project::Project, 
        time_entry::TimeEntry, 
        workspace::Workspace,
    },
    ui::components::StatefulList

};

use serde::{Serialize, Deserialize};
use tui::widgets::ListState; 

#[derive(Debug, Clone)]
pub struct App<'a> {
    pub title: &'a str, 
    pub should_quit: bool,
    pub config: Config,
    pub workspaces: StatefulList<Workspace>,
    pub projects: StatefulList<Project>,
    pub tags: StatefulList<Tag>, 
    pub time_entries: StatefulList<TimeEntry>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title, 
            should_quit: false, 
            config: confy::load("clockify").unwrap(), 
            workspaces: StatefulList::with_items(vec![]), 
            projects: StatefulList::with_items(vec![]),
            tags: StatefulList::with_items(vec![]), 
            time_entries: StatefulList::with_items(vec![]), 
        }
    }

    pub fn on_up(&mut self) {
        // TODO
    }

    pub fn on_down(&mut self) {
        // TODO
    }

    pub fn on_right(&mut self) {
        // TODO
    }

    pub fn on_left(&mut self) {
        // TODO
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
    
    pub fn on_tick(&mut self) {
        // Update
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: Option<String>, 
    pub workspace_id: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            base_url: String::from("https://api.clockify.me/api/v1"), 
            api_key: None, 
            workspace_id: None,
        }
    }
}

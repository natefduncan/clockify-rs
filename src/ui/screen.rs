use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers}
};
use tui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, Table, Row, TableState, Block},
    style::{Modifier, Style},
};
use reqwest::blocking::Client;
use crate::{
    clockify::App,
    api::{
        EndPoint,
        user::User, 
        time_entry::TimeEntry, 
        workspace::Workspace, project::Project, tag::Tag, task::Task
    }, 
    ui::{
        components::{StatefulList, Component, Id}, 
        Screen
    } 
};


// Template chunks
pub fn template_screen<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App) -> Vec<Rect> {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    return chunks; 
    
}

// Refresh workspaces
pub fn refresh_workspaces(client: &Client, app: &mut App, force: bool) {
    if app.workspaces.items.len() == 0 || force {
            app.workspaces = StatefulList::with_items(Workspace::list(client, &app.config, None).unwrap(), String::from("Select a workspace: "), false);
    }
}

// Refresh projects
pub fn refresh_projects(client: &Client, app: &mut App, force: bool) {
    if app.projects.items.len() == 0 || force {
            app.projects = StatefulList::with_items(Project::list(client, &app.config, None).unwrap(), String::from("Select a project: "), false);
    }
}

// Refresh tasks
pub fn refresh_tasks(client: &Client, app: &mut App, force: bool) {
    if app.tasks.items.len() == 0 || force {
        app.tasks = StatefulList::with_items(Task::list(client, &app.config, None).unwrap(), String::from("Select a task: "), false);
    }
}

// Refresh tags
pub fn refresh_tags(client: &Client, app: &mut App, force: bool) {
    if app.tags.items.len() == 0 || force {
        app.tags = StatefulList::with_items(Tag::list(client, &app.config, None).unwrap(), String::from("Select a tag: "), true);
    }
}

// Refresh Time Entries
pub fn refresh_time_entries(client: &Client, app: &mut App, force: bool) {
    if app.time_entries.items.len() == 0 || force {
        app.time_entries = StatefulList::with_items(TimeEntry::list(client, &app.config, None).unwrap(), String::from("Select a time entry: "), false);
    }
}

// Home
pub fn home<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]);
    
    // Display current time entry
    let current_entry_chunks = Layout::default()
        .constraints([
            Constraint::Length(1), // Project
            Constraint::Length(1), // Task
            Constraint::Length(1), // Tag
            Constraint::Length(1), // Description
            Constraint::Length(1), // Start
            Constraint::Length(1), // Stop
        ].as_ref())
        .split(chunks[1]);
    
    // Project
    let project: Option<&Project> = app.projects.get_selected_item();
    let project_text : String = match project {
        Some(project) => project.name.clone(), 
        None => String::from("")
    };
    f.render_widget(Paragraph::new(format!("{}: {}", "Project", project_text)), current_entry_chunks[0]); 
    // Task
    let task: Option<&Task> = app.tasks.get_selected_item();
    let task_text : String = match task {
        Some(t) => t.name.clone(), 
        None => String::from("")
    };
    f.render_widget(Paragraph::new(format!("{}: {}", "Task", task_text)), current_entry_chunks[1]);
    // Tag
    let tags: Vec<&Tag> = app.tags.get_selected_items();
    let tag_string = tags.iter().map(|x| x.to_string() + ", ").collect::<String>();
    let tag_string = tag_string.trim_end_matches(", ");
    f.render_widget(Paragraph::new(format!("{}: {}", "Tag", tag_string)), current_entry_chunks[2]); 
    // Description
    f.render_widget(Paragraph::new(format!("{}: {}", "Description", app.description.text.clone())), current_entry_chunks[3]); 

    if let Some(time_entry_id) = app.current_entry_id.clone() {
        let current_time = client.get(format!("{}/workspaces/{}/time-entries/{}", app.config.base_url, app.config.workspace_id.as_ref().unwrap().clone(), time_entry_id))
            .header("X-API-KEY", app.config.api_key.as_ref().unwrap().clone())
            .send().unwrap()
            .json::<TimeEntry>().unwrap();
        // Start
        f.render_widget(Paragraph::new(format!("{}: {}", "Start: ", current_time.time_interval.clone().unwrap().start.unwrap())), current_entry_chunks[4]); 
        // End
        let end : Option<String> = current_time.time_interval.clone().unwrap().end;
        if let Some(e) = end {
            f.render_widget(Paragraph::new(format!("{}: {}", "End: ", e)), current_entry_chunks[5]); 
        }
    }

    // If no user_id, send request
    if app.config.user_id.is_none() {
        let current_user = client.get(format!("{}{}", app.config.base_url, "/user"))
            .header("X-API-KEY", app.config.api_key.as_ref().unwrap().clone())
            .send().unwrap()
            .json::<User>().unwrap();
        app.config.user_id = current_user.id.clone();
    }

    // Force workspace selection
    if app.config.workspace_id.is_none() {
        app.current_screen = Screen::WorkspaceSelection;
    }
}

// Workspace selection
pub fn workspace_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]); 
    refresh_workspaces(client, app, false);
    app.workspaces.render(f, chunks[1]);
    
    // Key Event
    if let Some(event) = key {
        app.workspaces.key_event(event, &app.current_mode);
        match event.code {
            KeyCode::Enter => {
                app.config.workspace_id = app.workspaces.get_selected_item().unwrap().id.clone();
            }, 
            KeyCode::Char(c) => {
                match c {
                    'r' => {
                        refresh_workspaces(client, app, true);
                    }, 
                    _ => {}
                }
            }, 
           _ => {}
        }
    }
}

// Time Entry Selection
pub fn time_entry_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]); 
    refresh_time_entries(client, app, false);

    // Refresh data feeds
    refresh_workspaces(client, app, false);
    refresh_projects(client, app, false);
    refresh_tags(client, app, false);

    // Time Entry table
    let mut title = app.time_entries.title.clone();
    if !app.time_entries.search_text.is_empty() {
        title = format!("{}{}", title, app.time_entries.search_text);
    }
    let mut items = vec![];
    if app.time_entries.search_text.is_empty() {
        items = app.time_entries.items.iter().map(|x| x.clone()).collect(); // FIXME 
    } else {
        items = app.time_entries.search(&app.time_entries.search_text); 
    }
    let table = Table::new(
        items
        .iter()
        .map(|entry| {
            // Project name
            let mut project = String::new();
            if let Some(project_id) = &entry.project_id {
                if let Some(p) = app.projects.get_by_id(project_id.to_string()) {
                    project = p.to_string();
                }
            }
            // Task name
            let mut task = String::new();
            if let Some(task_id) = &entry.task_id {
                if let Some(t) = app.tasks.get_by_id(task_id.to_string()) {
                    task = t.to_string();
                }
            }
            // Tag names
            let mut tags = vec![];
            if let Some(tag_ids) = &entry.tag_ids {
                tags = tag_ids
                    .iter()
                    .map(|tag_id| {
                        if let Some(t) = app.tags.get_by_id(tag_id.clone()) {
                            t.to_string()
                        } else {
                            String::from("Unknown")
                        }
                    }).collect::<Vec<String>>();
            }
            let tag_string = tags.iter().map(|x| x.to_string() + ",").collect::<String>();
            let tag_string = tag_string.trim_end_matches(",");
            // Start, end, duration
            let mut start = String::new();
            let mut end = String::new();
            let mut duration = String::new(); 
            if let Some(time_interval) = &entry.time_interval {
                // Start
                if let Some(s) = &time_interval.start {
                    start = s.clone();
                }

                // End
                if let Some(e) = &time_interval.end {
                    end = e.clone();
                }

                // Duration
                if let Some(d) = &time_interval.duration {
                    duration = d.clone();
                }
            }
            return Row::new(vec![
                entry.description.as_ref().unwrap().clone(), 
                project,
                task, 
                tag_string.to_owned(), 
                start, 
                end,
                duration
            ]);
        })
    )
        .block(Block::default().title(title))
        .header(Row::new(vec!["Description", "Project", "Task", "Tag(s)", "Start", "End", "Duration"]))
        .widths(&[Constraint::Percentage(20), Constraint::Percentage(16), Constraint::Percentage(16), Constraint::Percentage(16), Constraint::Percentage(16), Constraint::Percentage(16), Constraint::Percentage(16)])
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::ITALIC).add_modifier(Modifier::UNDERLINED))
        .column_spacing(2);
    // Table State
    let mut state = TableState::default();
    if let Some(i) = app.time_entries.state.selected() {
        state.select(Some(i));
    }
    f.render_stateful_widget(table, chunks[1], &mut state);

    // Key Event
    if let Some(event) = key {
        match event.code {
            KeyCode::Enter => {
                let time_entry : &TimeEntry = app.time_entries.get_highlighted_item().unwrap();
                // Change project
                if let Some(project_id) = &time_entry.project_id {
                    app.projects.selected = vec![project_id.clone()];
                }
                // Change tags
                if let Some(tag_ids) = &time_entry.tag_ids {
                    app.tags.selected = tag_ids.clone();
                }
                // Change description
                app.description.text = time_entry.description.clone().unwrap().clone();
                
                // Change current_entry_id
                app.current_entry_id = time_entry.id.clone();

                // Change to home screen
                app.current_screen = Screen::Home;
            }, 
            KeyCode::Char(c) => {
                match c {
                    'r' => {
                        refresh_time_entries(client, app, true);
                        refresh_workspaces(client, app, true);
                        refresh_projects(client, app, true);
                        refresh_tags(client, app, true);
                    }, 
                    _ => {}
                }
            },
            _ => {}
        }
        app.time_entries.key_event(event, &app.current_mode);
    }
}

// Project Selection
pub fn project_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]);
    refresh_projects(client, app, false);

    app.projects.render(f, chunks[1]);

    // Key Event
    if let Some(event) = key {
        app.projects.key_event(event, &app.current_mode);
        match event.code {
            KeyCode::Char(c) => {
                match c {
                    'r' => {
                        refresh_projects(client, app, true);
                    },
                    _ => {}
                }
            }, 
            _ => {}
        }
   }
}

// Task Selection
pub fn task_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]);
    // Ensure that a project is set in the config
    refresh_projects(client, app, false);
    if let Some(project_id) = app.projects.get_selected_item() {
        app.config.project_id = Some(project_id.clone().id());
    } else {
        app.config.project_id = Some(app.projects.items.get(0).unwrap().clone().id()); 
    }
    refresh_tasks(client, app, false);
    app.tasks.render(f, chunks[1]);

    // Key Event
    if let Some(event) = key {
        app.tasks.key_event(event, &app.current_mode);
        match event.code {
            KeyCode::Char(c) => {
                match c {
                    'r' => {
                        refresh_tasks(client, app, true);
                    },
                    _ => {}
                }
            }, 
            _ => {}
        }
  }
}


// Tag Selection
pub fn tag_selection<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]);
    refresh_tags(client, app, false);
    app.tags.render(f, chunks[1]);

    // Key Event
    if let Some(event) = key {
        app.tags.key_event(event, &app.current_mode);
        match event.code {
            KeyCode::Char(c) => {
                match c {
                    'r' => {
                        refresh_tags(client, app, true);
                    },
                    _ => {}
                }
            }, 
            _ => {}
        }
   }
}

// Description Input
pub fn description_input<B: Backend>(f: &mut Frame<B>, client: &Client, app: &mut App, key: Option<KeyEvent>) {
    // App Title
    let chunks = template_screen(f, client, app);
    f.render_widget(Paragraph::new(app.to_string()), chunks[0]);

    // Description
    app.description.render(f, chunks[1]);

    // Key Event
    if let Some(event) = key {
        app.description.key_event(event, &app.current_mode); 
        match event.code {
            KeyCode::Enter => { app.current_screen = Screen::Home },
            _ => {}
        }
    }
    
}

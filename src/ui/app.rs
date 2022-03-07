use tui::widgets::ListState; 

pub struct StatefulList<T> {
    pub state: ListState, 
    pub items: Vec<T>
}

impl<T> StatefulList<T> {
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

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool, 
    // pub days: StatefulList<&'a str>, 
    // pub time_entries: StatefulList<&'a str>
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title, 
            should_quit: false, 
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


use crate::{
    db::Database,
    password::Password,
    view::View,
};
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;
use tui::{
    backend::Backend,
    Terminal,
};

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Details,
    Insert,
    Delete,
    Search
}

pub struct App {
    state: State,
    passwords: Vec<Password>,
    db: Database
}

impl App {
    pub fn new(key: &String) -> Result<Self, Box<dyn Error>> {
        let db = match Database::new(key) {
            Ok(db) => db,
            Err(err) => return Err(err)
        };

        let passwords = db.load_passwords()?;

        Ok(
            App {
                state: State::Details,
                passwords,
                db
            }
        )
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        let mut view = View::new();
        loop {
	        terminal.draw(|f| {
                view.draw_ui(f, &self).unwrap();
	        })?;
	    
            if let Event::Key(key) = event::read()? {
		        match key.code {
		            KeyCode::Char('q') => return Ok(()),
		            KeyCode::Up => {
                        if self.state == State::Details {
                            view.select_prev(self.passwords.len());
                        }
                    },
                    KeyCode::Down => {
                        if self.state == State::Details {
                            view.select_next(self.passwords.len());
                        }
                    },
                    KeyCode::Left | KeyCode::Right => view.toggle_focus_details(),
                    KeyCode::Tab => self.next_tab(),
                    KeyCode::Char(' ') => view.toggle_show_password(),
                    KeyCode::Insert => self.state = State::Insert,
                    KeyCode::Delete => self.state = State::Delete,
                    KeyCode::Char('c') => todo!(),
                    _ => {},
                }
            }
        }
    }

    fn next_tab(&mut self) {
        match self.state {
            State::Details => self.state = State::Insert,
            State::Insert => self.state = State::Delete,
            State::Delete => self.state = State::Search,
            State::Search => self.state = State::Details
        }
    }

    pub fn get_passwords(&self) -> &Vec<Password> {
        &self.passwords
    }

    pub fn get_num_passwords(&self) -> usize {
        self.passwords.len()
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_tab_index(&self) -> usize {
        match self.state {
            State::Details => 0,
            State::Insert => 1,
            State::Delete => 2,
            State::Search => 3
        }
    }
}

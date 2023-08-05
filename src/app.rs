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

#[derive(PartialEq)]
enum State {
    Menu,
    Entry,
    Delete,
    Search,
    Copy,
}

pub struct App {
    state: State,
    db: Database,
    passwords: Vec<Password>
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
                state: State::Menu,
                db,
                passwords
            }
        )
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        let mut view = View::new();
        loop {
	        terminal.draw(|f| {
            	match self.state {
                    State::Menu => {
                    	view.draw_ui(f, &self).unwrap();
                    },
                    State::Entry => {
                    	//let new_password = View::new_pw_handler();
                    	//app.safe.insert(&new_password)?;
                    	self.change_state(State::Menu);
                    },
                    State::Delete => {
                    	//let pass_id = View::del_pw_handler();
                    	//app.safe.delete(id)?;
                    	self.change_state(State::Menu);
                    },
                    State::Search => {
                    	todo!();
                    },
                    State::Copy => {
                    	todo!();
                    }
            	}
	        })?;
	    
            if let Event::Key(key) = event::read()? {
		        match key.code {
		            KeyCode::Char('q') => return Ok(()),
		            KeyCode::Up => {
                        if self.state == State::Menu {
                            view.select_prev(self.passwords.len());
                        }
                    },
                    KeyCode::Down => {
                        if self.state == State::Menu {
                            view.select_next(self.passwords.len());
                        }
                    },
                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),
                    _ => {},
                }
            }
        }
    }

    pub fn get_passwords(&self) -> &Vec<Password> {
        &self.passwords
    }

    fn change_state(&mut self, new_state: State) {
        self.state = new_state;
    }
}

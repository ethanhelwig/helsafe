use crate::{
    db::Database,
    password::Password,
    view::draw_ui,
};
use rusqlite::ErrorCode;
use crossterm::event::{self, Event, KeyCode};
use std::{
    process::exit,
    error::Error,
};
use tui::{
    backend::Backend,
    Terminal,
};

enum State {
    Menu,
    Entry,
    Delete,
    Search,
    Copy,
    Exit,
}

pub struct App {
    state: State,
    pub passwords: Vec<Password>,
    search_text: String,
}

impl App {
    pub fn new(key: &String) -> Result<Self, Box<dyn Error>> {
        let db = match Database::new(key) {
            Ok(db) => db,
            Err(err) => {
                if let Some(rusqlite_err) = err.downcast_ref::<rusqlite::Error>() {
                    if rusqlite_err.sqlite_error_code() == Some(ErrorCode::NotADatabase) {
                        println!("Error: HTTP 500 Internal Server Error");
                        exit(1);
                    }
                } else {
                    println!("{:?}", err);
                }

                exit(1);
            }
        };

        Ok(
            App {
                state: State::Menu,
                passwords: db.load_passwords()?,
                search_text: String::new(),
            }
        )
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
	        terminal.draw(|f| {
            	match self.state {
                    State::Menu => {
                    	let _next_state = draw_ui(f, &self).unwrap();
                    	//self.change_state(next_state);
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
                    },
                    State::Exit => exit(0),
            	}
	        });
	    
            if let Event::Key(key) = event::read()? {
		        match key.code {
		            KeyCode::Char('q') => exit(0),
		            KeyCode::Up => todo!(),
                    KeyCode::Down => todo!(),
                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),
                    _ => {},
                }
            }
        }
    }

    fn change_state(&mut self, new_state: State) {
        self.state = new_state;
    }
}

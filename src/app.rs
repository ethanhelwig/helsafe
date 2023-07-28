use crate::{
    password::Password,
    view::View,
};
use crossterm::event::{self, Event, KeyCode};
use std::{
    io::{self, BufRead, BufReader},
    collections::HashMap,
    error::Error,
    fs::File,
};
use rpassword;
use tui::{
    backend::Backend,
    layout::Rect,
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
    db: Database,
    passwords: Vec<Password>,
    search_txt: String,
}

impl App {
    pub fn new(key: &String) -> Result<Self, Box<dyn Error>> {
        let db = match Database::new(key) {
            Ok(db) => db,
            Err(err) => {
                if let Some(rusqlite_err) = err.downcast_ref::<rusqlite::Error>() {
                    println!("Error: HTTP 500 Internal Server Error");
                } else {
                    println!("{:?}", err);
                }

                exit(1);
            }
        };

        Ok(
            App {
                state: State::Menu,
                db,
                passwords: db.load_passwords()?,
                search_text: String::new(),
            }
        )
    }

    pub fn run() -> Result<(), Box<dyn Error>> {
        loop {
            match app.state {
                State::Menu => {
                    let next_state = View::menu_handler();
                    app.change_state(next_state);
                },
                State::Entry => {
                    let new_password = View::new_pw_handler();
                    app.safe.insert(&new_password)?;
                    app.change_state(State::Menu);
                },
                State::Delete => {
                    let pass_id = View::del_pw_handler();
                    app.safe.delete(id)?;
                    app.change_state(State::Menu);
                },
                State::Search => {
                    todo!();
                },
                State::Copy => {
                    todo!();
                },
                State::Exit => return Ok(()),
            }
        }
    }

    fn change_state(&mut self, new_state: State) {
        self.state = new_state;
    }
}

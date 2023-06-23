mod password;
mod helsafe;
mod cli;
mod db;

use crate::{
    password::Password,
    helsafe::Helsafe,
    cli::Cli
};
use rpassword;
use std::{
    error
};

pub enum OperationMode {
    Menu,
    Entry,
    Delete,
    Search,
    List,
    Copy,
    Exit
}

struct App {
    mode: OperationMode,
    safe: Helsafe,
    view: Cli
}

impl App {
    fn new(key: &String) -> Result<Self, Box<dyn error::Error>> {
        let helsafe = Helsafe::new(key)?;
        Ok(App {
            safe: helsafe,
            mode: OperationMode::Menu,
            view: Cli::new()
        })
    }

    fn change_mode(&mut self, new_mode: OperationMode) {
        self.mode = new_mode;
    }
}

fn main() {
    let key = rpassword::prompt_password("Enter password:").unwrap();
    let mut app = App::new(&key).unwrap();

    loop {
        match app.mode {
            OperationMode::Menu => {
                let choice = app.view.menu_handler();
                app.change_mode(choice);
            },
            OperationMode::Entry => {
                let new_password = app.view.new_entry_handler();
                app.safe.insert(&new_password).unwrap();
                app.change_mode(OperationMode::Menu);
            },
            OperationMode::Delete => {
                let id = app.view.handle_delete_request();

                match app.safe.delete(id) {
                    Ok(password) => {
                        app.view.handle_delete_success(&password);
                    }
                    Err(err) => {
                        app.view.handle_delete_failure(err);
                    }
                }

                app.change_mode(OperationMode::Menu);
            },
            OperationMode::Search => {
                println!("Search");
            },
            OperationMode::List => {
                let passwords = app.safe.get_passwords().unwrap();
                app.view.print_all(passwords);
                app.change_mode(OperationMode::Menu);
            },
            OperationMode::Copy => {
                println!("Copy");
            }
            OperationMode::Exit => {
                println!("Exitting");
                std::process::exit(1);
            }
        }
    }
}

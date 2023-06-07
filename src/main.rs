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
    io::{self, Write, stdin}
};
/*
use crate::password::Password;
use crate::helsafe::Helsafe;
use cli_tables::Table;
use std::{io::{self, Write, stdin}, error::Error};
use rpassword;*/

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
    fn new(key: &String) -> Self {
        App {
            safe: Helsafe::new(key),
            mode: OperationMode::Menu,
            view: Cli::new()
        }
    }

    fn change_mode(&mut self, new_mode: OperationMode) {
        self.mode = new_mode;
    }
}

fn main() {
    let key = rpassword::prompt_password("Enter password:").unwrap();
    let mut app = App::new(&key);

    loop {
        match app.mode {
            OperationMode::Menu => {
                let choice = app.view.menu_handler();
                app.change_mode(choice);
            },
            OperationMode::Entry => {
                let new_password = app.view.new_entry_handler();
                app.safe.insert(&new_password);
                app.change_mode(OperationMode::Menu);
            },
            OperationMode::Delete => {
                let id = app.view.delete_handler();

                let result = app.safe.delete(id);
                match result {
                    Ok(()) => {
                        println!("Password deleted successfully.");
                    }
                    Err(err) => {
                        println!("Failed to delete password: {}", err);
                    }
                }

                app.change_mode(OperationMode::Menu);
            },
            OperationMode::Search => {
                println!("Search");
            },
            OperationMode::List => {
                println!("\nList:");

                app.safe.get_passwords();
                println!("{}", app.safe);

                print!("( Press enter to continue )");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut input: String = String::new();
                stdin().read_line(&mut input).expect("Failed to read input");

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
  

/*use rusqlite::ErrorCode;
use cli_tables::Table;
use std::{io, thread, time::Duration};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{List, ListItem, ListState, Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    text::{Span, Spans},
    Terminal,
    Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::db::Database;
mod db;

enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    List,
    Delete
}

#[derive(Clone, Debug)]
pub struct Password {
    id: usize,
    title: String,
    username: String,
    password: String
}

struct Helsafe {
    db: Database,
    passwords: Vec<Password>,
    mode: InputMode,
    search_txt: String,
    search_list: Vec<Password>,
    new_title: String,
    new_username: String,
    new_password: String,
}

impl Password {
    pub fn new(title: String, username: String, password: String) -> Password {
        Password {
            id: 0,
            title,
            username,
            password
        }
    }

    pub fn new_with_id(id: usize, title: String, username: String, password: String) -> Password {
        Password {
            id,
            title,
            username,
            password
        }
    }
}

impl Helsafe {
    pub fn new(key: String) -> Helsafe {
        let db = match Database::new(key) {
            Ok(db) => db,
            Err(e) => {
                if e.sqlite_error_code().unwrap() == ErrorCode::NotADatabase {
                    println!("Passphrase is not valid!");
                    std::process::exit(1);
                } else {
                    println!("{}", e.to_string());
                    std::process::exit(1);
                }
            }
        };

        let passwords = db.get_passwords();

        Helsafe {
            db,
            passwords,
            mode: InputMode::Normal,
            search_txt: String::new(),
            search_list: vec![],
            new_title: String::new(),
            new_username: String::new(),
            new_password: String::new()
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn insert(&mut self) {
        let password: Password = Password::new(
            self.new_title.to_owned(),
            self.new_username.to_owned(),
            self.new_password.to_owned()
        );
        self.db.insert(&password);
        self.passwords.push(password);
        self.change_mode(InputMode::Normal);
    }

    pub fn search(&mut self) {
        self.search_list = self.passwords.clone().into_iter()
            .filter(|item| item.title.starts_with(&self.search_txt.to_owned()))
            .collect();
    }

    
}

fn main() -> Result<(), io::Error> {
    let helkey: String = rpassword::prompt_password("Enter HELKEY:").unwrap();
    let mut helsafe = Helsafe::new(helkey);
    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut helsafe);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
    /*
    let key = "password".to_string();
    let db = match Database::new(key) {
        Ok(db) => db,
        Err(e) => {
            if e.sqlite_error_code().unwrap() == ErrorCode::NotADatabase {
                println!("Passphrase is not valid!");
                std::process::exit(1);
            } else {
                println!("{}", e.to_string());
                std::process::exit(1);
            }
        }
    };

    let options: Vec<Vec<String>> = vec![
        vec!["#".to_string(), "Command".to_string(), "Description".to_string()],
        vec!["0".to_string(), "Insert".to_string(), "Enter a new password".to_string()],
        vec!["1".to_string(), "List".to_string(), "List all passwords".to_string()],
        vec!["2".to_string(), "Delete".to_string(), "Delete a password".to_string()],
    ];
    let mut table: Table = Table::new(&options);
    let mut input: String = String::new();
    let table_str = table.to_string();
    loop {
        println!("{}", table_str);
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        if input.trim().parse::<u32>().unwrap() == 0 {
            let mut title: String = String::new();
            let mut username: String = String::new();
            let mut password: String = String::new();

            println!("title: ");
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read from stdin");
            println!("username: ");
            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read from stdin");
            println!("password: ");
            io::stdin()
                .read_line(&mut password)
                .expect("Failed to read from stdin");

            println!("{},{},{}", title, username, password);

            let password = Password::new(
                title,
                username,
                password
            );
            
            db.insert(&password);
        }
        else if input.trim().parse::<u32>().unwrap() == 1 {
            let passwords = db.get_passwords();
            println!("{:?}", passwords);
        }
        else if input.trim().parse::<u32>().unwrap() == 2 {
            let mut id_str: String = String::new();
            println!("id: ");
            io::stdin()
                .read_line(&mut id_str)
                .expect("Failed to read from stdin");
            let id: usize = id_str.trim().parse().expect("Input not an integer"); 
            db.delete(id);
        }
        else {
            break;
        }
    }
    */
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, helsafe: &mut Helsafe) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, helsafe))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, helsafe: &mut Helsafe) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        //.margin(1)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ]
            .as_ref()
        )
        .split(f.size());

    let block = Block::default()
        .title("Menu")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title("Vault")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    // Print out the passwords
    let passwords = &helsafe.passwords; // Assuming `get_passwords` retrieves the passwords from `helsafe`
    let password_texts: Vec<ListItem> = passwords
        .iter()
        .map(|password| ListItem::new(Spans::from(vec![Span::raw(password.password.clone())])))
        .collect();
    let password_list = List::new(password_texts)
        .block(
            Block::default()
                .title("Passwords")
                .borders(Borders::ALL)
        );
    f.render_widget(password_list, chunks[1]);
}*/
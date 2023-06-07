use std::{
    io::{self, Write, Stdin},
};
use cli_tables::Table;
use rpassword;
use crate::{
    OperationMode,
    password::Password
};

pub struct Cli {
    stdin: Stdin,
    str_vec: Vec<Vec<String>>
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            stdin: io::stdin(),
            str_vec: Vec::new()
        }
    }

    pub fn menu_handler(&mut self) -> OperationMode {
        println!("\nMenu:");

        self.str_vec = vec![
            vec!["#".to_string(), "Options".to_string()],
            vec!["0".to_string(), "Add password".to_string()],
            vec!["1".to_string(), "Delete password".to_string()],
            vec!["2".to_string(), "Search password".to_string()],
            vec!["3".to_string(), "List passwords".to_string()],
            vec!["4".to_string(), "Copy password".to_string()],
            vec!["5".to_string(), "Exit".to_string()]
        ];

        // create an ascii table
        let mut table = Table::new(&self.str_vec);
        let mut input = String::new();

        loop {
            // print the table
            println!("{}", table.to_string());

            // user input
            input.clear();
            self.stdin
                .read_line(&mut input)
                .expect("Failed to read input");
            
            // trim and parse the value
            if let Ok(value) = input.trim().parse::<usize>() {
                match value {
                    0 => return OperationMode::Entry,
                    1 => return OperationMode::Delete,
                    2 => return OperationMode::Search,
                    3 => return OperationMode::List,
                    4 => return OperationMode::Copy,
                    5 => return OperationMode::Exit,
                    _ => {
                        println!("Invalid input. Please enter a valid option.");
                        continue;
                    }
                }
            } else {
                println!("Invalid input. Please enter a valid option.");
                continue;
            }
        }
    }

    pub fn new_entry_handler(&mut self) -> Password{
        let mut headers = vec![
            "title".to_string(),
            "username".to_string(),
            "password".to_string(),
            "email".to_string(),
            "recovery_codes".to_string(),
            "access_tokens".to_string(),
            "notes".to_string(),
        ];
        let mut title = String::new();
        let mut username = String::new();
        let mut password = String::new();
        let mut email = String::new();
        let mut recovery_codes = String::new();
        let mut access_tokens = String::new();
        let mut notes = String::new();

        self.str_vec = vec![
            headers,
            vec![
                title,
                username,
                password,
                email,
                recovery_codes,
                access_tokens,
                notes,
            ]
        ];

        println!("\n( Password creation manager )");
        let mut table = Table::new(&self.str_vec);
        println!("{}", table.to_string());

        print!("Title: ");
        let mut title = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut title)
            .expect("Failed to read input.");
        let title = title.trim();
        

        print!("Username: ");
        let mut username = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut username)
            .expect("Failed to read input.");
        let username = username.trim();

        let password = rpassword::prompt_password("Password: ").unwrap();
        print!("*************\n");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");

        print!("Email: ");
        let mut email = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut email)
            .expect("Failed to read input.");
        let email = email.trim();

        print!("Recovery Codes: ");
        let mut recovery_codes = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut recovery_codes)
            .expect("Failed to read input.");
        let recovery_codes = recovery_codes.trim();

        print!("Access Tokens: ");
        let mut access_tokens = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut access_tokens)
            .expect("Failed to read input.");
        let access_tokens = access_tokens.trim();

        print!("Notes: ");
        let mut notes = String::new();
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut notes)
            .expect("Failed to read input.");
        let notes = notes.trim();

        // Create the Password struct using the trimmed values
        Password {
            id: 0,
            title: title.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            recovery_codes: recovery_codes.to_string(),
            access_tokens: access_tokens.to_string(),
            notes: notes.to_string(),
        }
    }

    pub fn delete_handler(&mut self) -> usize {
        let mut input = String::new();

        println!("\n( You cannot undo this action )");
        print!("Id: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut input)
            .expect("Failed to read input.");
        let trimmed_input = input.trim();

        if let Ok(id) = trimmed_input.parse::<usize>() {
            return id;
        } else {
            println!("Failed to parse integer.");
            std::process::exit(1);
        }

        /*let delete_result: Result<(), Box<dyn Error>> = safe.delete(id);
        match delete_result {
            Ok(()) => {
                println!("Password deleted successfully.");
            }
            Err(err) => {
                println!("Failed to delete password: {}", err);
            }
        }*/
    }
}
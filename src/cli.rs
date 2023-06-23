use std::{
    io::{self, Write, Stdin},
    error
};
use cli_tables::table_from;
use rpassword;
use crate::{
    OperationMode,
    password::Password
};

pub struct Cli {
    stdin: Stdin,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            stdin: io::stdin(),
        }
    }

    pub fn menu_handler(&mut self) -> OperationMode {
        println!("\nMenu:");

        // create an ascii table
        let options = vec![
            vec!["#", "Options"],
            vec!["0", "Add password"],
            vec!["1", "Delete password"],
            vec!["2", "Search password"],
            vec!["3", "List passwords"],
            vec!["4", "Copy password"],
            vec!["5", "Exit"]
        ];
        let table = table_from(&options);

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

    pub fn print_all(&self, passwords: &Vec<Password>) {
        println!("\nList:");

        let mut table_vec = Vec::new();
        let field_names = vec![
            "Id",
            "Title",
            "Username",
            "Password",
            "Email",
            "Recovery Codes",
            "Access Tokens",
            "Notes"
        ];
        table_vec.push(field_names);

        for password in passwords {
            table_vec.push(vec![
                password.id.as_str(),
                password.title.as_str(),
                password.username.as_str(),
                password.password.as_str(),
                password.email.as_str(),
                password.recovery_codes.as_str(),
                password.access_tokens.as_str(),
                password.notes.as_str(),
            ]);
        }

        println!("{}", table_from(&table_vec));
        print!("Do you want to continue? [Y/n]");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input: String = String::new();
        self.stdin.read_line(&mut input).expect("Failed to read input");
    }

    pub fn new_entry_handler(&mut self) -> Password{
        let mut table = Vec::new();
        let headers = vec![
            "Title".to_string(),
            "Username".to_string(),
            "Password".to_string(),
            "Email".to_string(),
            "Recovery_codes".to_string(),
            "Access_tokens".to_string(),
            "Notes".to_string(),
        ];
        table.push(headers);
        
        table.push(vec![String::new(); 7]);

        println!("\n( Password creation manager )");

        print!("Title: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut table[1][0])
            .expect("Failed to read input.");
        table[1][0] = table[1][0].trim().to_string();
        

        print!("Username: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut table[1][1])
            .expect("Failed to read input.");
        table[1][1] = table[1][1].trim().to_string();

        table[1][2] = rpassword::prompt_password("Password: ").unwrap();
        
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        table[1][2] = table[1][2].trim().to_string();

        print!("Email: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut table[1][3])
            .expect("Failed to read input.");
        table[1][3] = table[1][3].trim().to_string();

        print!("Recovery Codes: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut table[1][4])
            .expect("Failed to read input.");
        table[1][4] = table[1][4].trim().to_string();

        print!("Access Tokens: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut table[1][5])
            .expect("Failed to read input.");
        table[1][5] = table[1][5].trim().to_string();

        print!("Notes: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut table[1][6])
            .expect("Failed to read input.");
        table[1][6] = table[1][6].trim().to_string();

        // Create the Password struct using the trimmed values
        Password {
            id: "0".to_string(),
            title: table[1][0].clone(),
            username: table[1][1].clone(),
            password: table[1][2].clone(),
            email: table[1][3].clone(),
            recovery_codes: table[1][4].clone(),
            access_tokens: table[1][5].clone(),
            notes: table[1][6].clone(),
        }
    }

    pub fn handle_delete_request(&self) -> usize {
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
    }

    pub fn handle_delete_success(&self, password: &Password) {
        println!("Success: {} deleted", password.title);
    }

    pub fn handle_delete_failure(&self, err: Box<dyn error::Error>) {
        println!("Failed: {}", err);
    }
}
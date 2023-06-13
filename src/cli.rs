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
        let mut table = Table::new();
        let options = vec![
            vec!["#", "Options"],
            vec!["0", "Add password"],
            vec!["1", "Delete password"],
            vec!["2", "Search password"],
            vec!["3", "List passwords"],
            vec!["4", "Copy password"],
            vec!["5", "Exit"]
        ];
        table.push_rows(&options).unwrap();

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
        let mut table = Table::new();
        let headers = vec![
            "Title",
            "Username",
            "Password",
            "Email",
            "Recovery_codes",
            "Access_tokens",
            "Notes",
        ];
        table.push_row(&headers).unwrap();
        
        let mut entry = vec![String::new(); 7];

        println!("\n( Password creation manager )");
        println!("{}", table.to_string());

        print!("Title: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut entry[0])
            .expect("Failed to read input.");
        entry[0] = entry[0].trim().to_string();
        table.push_row_string(&entry).unwrap();
        println!("{}", table.to_string());
        

        print!("Username: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut entry[1])
            .expect("Failed to read input.");
        entry[1] = entry[1].trim().to_string();
        table.delete_record(1).unwrap();
        table.push_row_string(&entry).unwrap();

        entry[2] = rpassword::prompt_password("Password: ").unwrap();
        print!("*************\n");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        entry[2] = entry[2].trim().to_string();
        table.delete_record(1).unwrap();
        table.push_row_string(&entry).unwrap();

        print!("Email: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut entry[3])
            .expect("Failed to read input.");
        entry[3] = entry[3].trim().to_string();
        table.delete_record(3).unwrap();
        table.push_row_string(&entry).unwrap();

        print!("Recovery Codes: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut entry[4])
            .expect("Failed to read input.");
        entry[4] = entry[4].trim().to_string();
        table.delete_record(1).unwrap();
        table.push_row_string(&entry).unwrap();

        print!("Access Tokens: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut entry[5])
            .expect("Failed to read input.");
        entry[5] = entry[5].trim().to_string();
        table.delete_record(1).unwrap();
        table.push_row_string(&entry).unwrap();

        print!("Notes: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");
        self.stdin
            .read_line(&mut entry[6])
            .expect("Failed to read input.");
        entry[6] = entry[6].trim().to_string();
        table.delete_record(1).unwrap();
        table.push_row_string(&entry).unwrap();

        // Create the Password struct using the trimmed values
        Password {
            id: 0,
            title: entry[0].clone(),
            username: entry[1].clone(),
            password: entry[2].clone(),
            email: entry[3].clone(),
            recovery_codes: entry[4].clone(),
            access_tokens: entry[5].clone(),
            notes: entry[6].clone(),
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
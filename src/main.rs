use rusqlite::ErrorCode;
use cli_tables::Table;
use std::io;
use crate::db::Database;
mod db;

#[derive(Clone, Debug)]
pub struct Password {
    id: usize,
    title: String,
    username: String,
    password: String
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

fn main() {
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
            let passwords = db.load();
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
}

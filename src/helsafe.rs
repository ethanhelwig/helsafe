use crate::db::Database;
use crate::password::Password;
use cli_tables::Table;
use std::{error::Error, process::exit, fmt::{self, Display, Formatter}};
use rusqlite::ErrorCode;

#[derive(Debug)]
pub struct Helsafe {
    db: Database,
    passwords: Vec<Password>,
    pub search_txt: String,
    pub search_list: Vec<Password>,
    pub new_title: String,
    pub new_username: String,
    pub new_password: String,
    pub new_email: String,
    pub new_recovery_codes: String,
    pub new_access_tokens: String,
    pub new_notes: String,
}

impl Helsafe {
    pub fn new(key: &String) -> Helsafe {
        let db: Database = match Database::new(key.to_owned()) {
            Ok(db) => db,
            Err(e) => {
                if let Some(rusqlite_err) = e.downcast_ref::<rusqlite::Error>() {
                    if rusqlite_err.sqlite_error_code() == Some(ErrorCode::NotADatabase) {
                        println!("Passphrase is not valid!");
                        exit(1);
                    }
                }
                
                println!("{}", e.to_string());
                exit(1);
            }
        };

        let passwords: Vec<Password> = db.get_passwords();

        Helsafe {
            db,
            passwords,
            search_txt: String::new(),
            search_list: vec![],
            new_title: String::new(),
            new_username: String::new(),
            new_password: String::new(),
            new_email: String::new(),
            new_recovery_codes: String::new(),
            new_access_tokens: String::new(),
            new_notes: String::new(),
        }
    }

    pub fn insert(&mut self, password: &Password) {
        self.passwords.push(password.to_owned());
        self.db.insert(password);
    }

    pub fn delete(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.passwords.get(id) {
            self.passwords.remove(id);
            self.db.delete(&id);
            Ok(())
        } else {
            Err("Invalid password id. Unable to complete request.".into())
        }
    }

    pub fn get_passwords(&mut self) {
        self.passwords = self.db.get_passwords();
    }
}

impl Display for Helsafe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut table_arr: Vec<Vec<String>> = Vec::new();

        let field_names = vec![
            "Id".to_string(),
            "Title".to_string(),
            "Username".to_string(),
            "Password".to_string(),
            "Email".to_string(),
            "Recovery Codes".to_string(),
            "Access Tokens".to_string(),
            "Notes".to_string()
        ];

        table_arr.push(field_names);
        for password in &self.passwords {
            let row: Vec<String> = vec![
                password.id.to_string(),
                password.title.clone(),
                password.username.clone(),
                password.password.clone(),
                password.email.clone(),
                password.recovery_codes.clone(),
                password.access_tokens.clone(),
                password.notes.clone(),
            ];
            table_arr.push(row);
        }

        let mut table: Table = Table::new(&table_arr);
        write!(f, "{}", table.to_string())
    }
}
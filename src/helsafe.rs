use crate::db::Database;
use crate::password::Password;
use cli_tables::table_from;
use std::{error, process::exit, fmt::{self, Display, Formatter}};
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
    pub fn new(key: &String) -> Result<Helsafe, Box<dyn error::Error>> {
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

        let passwords: Vec<Password> = db.get_passwords()?;

        Ok(Helsafe {
            db,
            passwords,
            search_txt: String::new(),
            search_list: Vec::new(),
            new_title: String::new(),
            new_username: String::new(),
            new_password: String::new(),
            new_email: String::new(),
            new_recovery_codes: String::new(),
            new_access_tokens: String::new(),
            new_notes: String::new(),
        })
    }

    pub fn insert(&mut self, password: &Password) -> Result<(), rusqlite::Error> {
        self.passwords.push(password.to_owned());
        self.db.insert(password)?;
        Ok(())
    }

    pub fn delete(&mut self, id: usize) -> Result<Password, Box<dyn error::Error>> {
        match self.passwords.get(id) {
            Some(..) => {
                self.db.delete(&id)?;
                let deleted_password = self.passwords.remove(id);
                Ok(deleted_password)
            }
            None => {
                Err("invalid password id, unable to complete request".into())
            }
        }
    }

    pub fn get_passwords(&mut self) -> Result<&Vec<Password>, rusqlite::Error> {
        self.passwords = self.db.get_passwords()?;
        Ok(&self.passwords)
    }
}

impl Display for Helsafe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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

        for password in &self.passwords {
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

        write!(f, "{}", table_from(&table_vec))
    }
}
use rusqlite::{Connection, DatabaseName, params};
use std::{path::PathBuf, error, io};
use crate::password::Password;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(key: &String) -> Result<Database, Box<dyn error::Error>> {
        let db_path: PathBuf = Self::establish_path()?;

        let conn: Connection = Connection::open(db_path)?;

        // set password to our database. without this passphrase database is not readable
        conn.pragma_update(Some(DatabaseName::Main), "KEY", key)?;

        let db: Database = Database { conn };
        db.create_table()?;

        Ok(db)
    }

    fn establish_path() -> Result<PathBuf, io::Error> {
        let config_dir: PathBuf = dirs::config_dir().unwrap_or_else(|| {
            // Fallback directory if config_dir is None
            let fallback_dir: PathBuf = dirs::home_dir().unwrap();
            fallback_dir.join(".config")
        });

        let helsafe_dir: PathBuf = config_dir.join(".helsafe");
        let db_path: PathBuf = helsafe_dir.join("helsafe.sqlite");

        // Create .config directory if it doesn't exist
        if !config_dir.exists() {
            std::fs::create_dir(&config_dir)?;
        }

        // Create .helsafe directory if it doesn't exist
        if !helsafe_dir.exists() {
            std::fs::create_dir(&helsafe_dir)?;
        }

        // Create database file if it doesn't exist
        if !db_path.exists() {
            std::fs::File::create(&db_path)?;
        }

        Ok(db_path)
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS passwords(
                id             INTEGER PRIMARY KEY AUTOINCREMENT,
                title          TEXT    NOT NULL,
                username       TEXT    NOT NULL,
                password       TEXT    NOT NULL,
                email          TEXT    NOT NULL,
                recovery_codes TEXT    NOT NULL,
                access_tokens  TEXT    NOT NULL,
                notes          TEXT    NOT NULL
            )"
        )?;
        Ok(())
    }

    pub fn load_passwords(&self) -> Result<Vec<Password>, rusqlite::Error> {
        let mut statement = self.conn.prepare("SELECT * FROM passwords")?;
        let items: Vec<Password> = statement.query_map([], |row| {
            let password = Password::new_with_id(
                row.get("id").unwrap(),
                row.get("title").unwrap(),
                row.get("username").unwrap(),
                row.get("password").unwrap(),
                row.get("email").unwrap(),
                row.get("recovery_codes").unwrap(),
                row.get("access_tokens").unwrap(),
                row.get("notes").unwrap(),
            );
            println!("{}", password);
            Ok(password)
        })?.map(|i| i.unwrap()).collect();
        Ok(items)
    }

    pub fn insert(&self, password: &Password) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO passwords (title, username, password, email, recovery_codes, access_tokens, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![password.title, password.username, password.password, password.email, password.recovery_codes, password.access_tokens, password.notes]
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn update(&self, id: usize, password: &Password) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "UPDATE passwords SET title=?1, username=?2, password=?3 email=?4 recovery_codes=?5 access_tokens=?6 notes=?7 WHERE id=?8",
            params![password.title, password.username, password.password, password.email, password.recovery_codes, password.access_tokens, password.notes, id]
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &usize) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "DELETE FROM passwords WHERE id=?",
            params![*id]
        )?;
        Ok(())
    }
}

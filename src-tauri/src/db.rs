use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub sender: String,
    pub timestamp: i64,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> Result<Self, rusqlite::Error> {
        let app_dir = app_handle.path().app_data_dir()
            .expect("Failed to get app data dir");
		println!("App dir: {:?}", app_dir);
        std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
        let db_path = PathBuf::from(app_dir).join("messages.db");

        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                text TEXT NOT NULL,
                sender TEXT NOT NULL CHECK (sender IN ('user', 'bot')),
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn save_message(&self, id: &str, text: &str, sender: &str) -> Result<(), rusqlite::Error> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn.execute(
            "INSERT INTO messages (id, text, sender, timestamp) VALUES (?1, ?2, ?3, ?4)",
            params![id, text, sender, timestamp],
        )?;
        Ok(())
    }

    pub fn get_messages(&self) -> Result<Vec<Message>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, text, sender, timestamp FROM messages ORDER BY timestamp ASC"
        )?;

        let messages = stmt.query_map([], |row| {
            Ok(Message {
                id: row.get(0)?,
                text: row.get(1)?,
                sender: row.get(2)?,
                timestamp: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(messages)
    }

    pub fn clear_messages(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute("DELETE FROM messages", [])?;
        Ok(())
    }
}

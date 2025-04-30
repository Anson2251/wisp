use rusqlite::{params, Connection, Statement};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use super::statement::leak_stmt;

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Invalid message role: {0}")]
    InvalidRole(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "bot")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

impl std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageRole::User => write!(f, "user"),
            MessageRole::Assistant => write!(f, "bot"),
            MessageRole::System => write!(f, "system"),
        }
    }
}

impl TryFrom<String> for MessageRole {
    type Error = MessageError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "user" => Ok(MessageRole::User),
            "bot" => Ok(MessageRole::Assistant),
            "system" => Ok(MessageRole::System),
            s => Err(MessageError::InvalidRole(s.to_string())),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub sender: MessageRole,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<u8>>,
}

pub struct Messages {
    conn: Connection,
    insert_stmt: Statement<'static>,
    get_stmt: Statement<'static>,
    update_text_stmt: Statement<'static>,
    update_sender_stmt: Statement<'static>,
    delete_stmt: Statement<'static>,
}

impl Messages {
    pub const TABLE_NAME: &'static str = "messages";

    pub fn new(db_path: &str) -> Result<Self, MessageError> {
		let conn = Connection::open(db_path)?;
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                text TEXT NOT NULL,
                sender TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                tokens INTEGER,
                embedding BLOB
            )",
                Self::TABLE_NAME
            ),
            [],
        )?;

        unsafe {
            let insert_stmt = leak_stmt(conn.prepare(&format!(
                "INSERT INTO {} (id, text, sender, timestamp) VALUES (?1, ?2, ?3, ?4)",
                Self::TABLE_NAME
            ))?);

            let get_stmt = leak_stmt(conn.prepare(&format!(
                "SELECT id, text, sender, timestamp FROM {} WHERE id = ?1",
                Self::TABLE_NAME
            ))?);

            let update_text_stmt = leak_stmt(conn.prepare(&format!(
                "UPDATE {} SET text = ?2 WHERE id = ?1",
                Self::TABLE_NAME
            ))?);

            let update_sender_stmt = leak_stmt(conn.prepare(&format!(
                "UPDATE {} SET sender = ?2 WHERE id = ?1",
                Self::TABLE_NAME
            ))?);

            let delete_stmt = leak_stmt(
                conn.prepare(&format!("DELETE FROM {} WHERE id = ?1", Self::TABLE_NAME))?,
            );

            Ok(Self {
                conn,
                insert_stmt,
                get_stmt,
                update_text_stmt,
                update_sender_stmt,
                delete_stmt,
            })
        }
    }

    pub fn add(&mut self, id: &str, text: &str, sender: &str, tokens: Option<i32>, embedding: Option<Vec<u8>>) -> Result<(), MessageError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.insert_stmt
            .execute(params![id, text, sender, timestamp, tokens, embedding])?;
        Ok(())
    }

    pub fn add_batch(&mut self, messages: &[(&str, &str, &str, Option<i32>, Option<Vec<u8>>)]) -> Result<(), MessageError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(&format!(
                "INSERT INTO {} (id, text, sender, timestamp, tokens, embedding) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                Self::TABLE_NAME
            ))?;

            for (id, text, sender, tokens, embedding) in messages {
                stmt.execute(params![id, text, sender, timestamp, tokens, embedding])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get(&mut self, id: &str) -> Result<Message, MessageError> {
        let row = self.get_stmt.query_row(params![id], |row| {
            let sender_str: String = row.get(2)?;
            let sender = MessageRole::try_from(sender_str)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
            Ok(Message {
                id: row.get(0)?,
                text: row.get(1)?,
                sender,
                timestamp: row.get(3)?,
                tokens: row.get(4)?,
                embedding: row.get(5)?,
            })
        })?;
        Ok(row)
    }

    pub fn list(&mut self, limit: i64, offset: i64) -> Result<Vec<Message>, MessageError> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, text, sender, timestamp, tokens, embedding FROM {} ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2",
            Self::TABLE_NAME
        ))?;

        let messages = stmt
            .query_map(params![limit, offset], |row| {
                let sender_str: String = row.get(2)?;
                let sender = MessageRole::try_from(sender_str)
                    .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
                Ok(Message {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    sender,
                    timestamp: row.get(3)?,
                    tokens: row.get(4)?,
                    embedding: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()
            .map_err(MessageError::from)?;

        Ok(messages)
    }

    pub fn update_text(&mut self, id: &str, text: &str) -> Result<(), MessageError> {
        self.update_text_stmt.execute(params![id, text])?;
        Ok(())
    }

    pub fn update_sender(&mut self, id: &str, sender: MessageRole) -> Result<(), MessageError> {
        self.update_sender_stmt
            .execute(params![id, sender.to_string()])?;
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> Result<(), MessageError> {
        self.delete_stmt.execute(params![id])?;
        Ok(())
    }

    pub fn delete_batch(&mut self, ids: &[&str]) -> Result<(), MessageError> {
        let tx = self.conn.transaction()?;
        for id in ids {
            self.delete_stmt.execute(params![id])?;
        }
        tx.commit()?;
        Ok(())
    }

}

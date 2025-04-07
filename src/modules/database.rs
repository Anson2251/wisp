use rusqlite::{Connection, params, OptionalExtension};
use chrono::{DateTime, Utc};
use std::path::Path;
use openai_api_rs::v1::chat_completion;

#[derive(Debug)]
#[derive(Clone)]
pub struct Conversation {
    pub id: i64,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let path = Path::new("wisp.db");
        let conn = Connection::open(path)?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY,
                conversation_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY(conversation_id) REFERENCES conversations(id)
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn create_conversation(&self, title: &str) -> Result<i64, rusqlite::Error> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO conversations (title, created_at, updated_at) VALUES (?1, ?2, ?3)",
            params![title, now, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_conversation(&self, id: i64) -> Result<Option<Conversation>, rusqlite::Error> {
        self.conn.query_row(
            "SELECT id, title, created_at, updated_at FROM conversations WHERE id = ?1",
            params![id],
            |row| {
                Ok(Conversation {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
                        .with_timezone(&Utc),
                })
            },
        ).optional()
    }

    pub fn list_conversations(&self) -> Result<Vec<Conversation>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, created_at, updated_at FROM conversations ORDER BY updated_at DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
                    .with_timezone(&Utc),
            })
        })?;

        rows.collect()
    }

    pub fn update_conversation_title(&self, id: i64, title: &str) -> Result<(), rusqlite::Error> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE conversations SET title = ?1, updated_at = ?2 WHERE id = ?3",
            params![title, now, id],
        )?;
        Ok(())
    }

    pub fn delete_conversation(&self, id: i64) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "DELETE FROM messages WHERE conversation_id = ?1",
            params![id],
        )?;
        self.conn.execute(
            "DELETE FROM conversations WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn add_message(
        &self,
        conversation_id: i64,
        message: &chat_completion::ChatCompletionMessage,
    ) -> Result<(), rusqlite::Error> {
        let now = Utc::now().to_rfc3339();
        let role = match message.role {
            chat_completion::MessageRole::user => "user",
            chat_completion::MessageRole::assistant => "assistant",
            _ => "system",
        };
        
        let content = match &message.content {
            chat_completion::Content::Text(text) => text,
            _ => "", // Handle other content types if needed
        };

        self.conn.execute(
            "INSERT INTO messages (conversation_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![conversation_id, role, content, now],
        )?;

        // Update conversation's updated_at
        self.conn.execute(
            "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
            params![now, conversation_id],
        )?;

        Ok(())
    }

    pub fn get_messages(
        &self,
        conversation_id: i64,
    ) -> Result<Vec<chat_completion::ChatCompletionMessage>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT role, content FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC"
        )?;
        
        let rows = stmt.query_map(params![conversation_id], |row| {
            let role: String = row.get(0)?;
            let content: String = row.get(1)?;
            
            Ok(chat_completion::ChatCompletionMessage {
                role: match role.as_str() {
                    "user" => chat_completion::MessageRole::user,
                    "assistant" => chat_completion::MessageRole::assistant,
                    _ => chat_completion::MessageRole::system,
                },
                content: chat_completion::Content::Text(content),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            })
        })?;

        rows.collect()
    }
}

use rusqlite::params;
use super::DbPool;

use std::time::{SystemTime, UNIX_EPOCH};
use super::types::{MessageError, MessageRole, Message};

pub struct Messages {
    pool: DbPool,
}

#[allow(unused)]
impl Messages {
    pub const TABLE_NAME: &'static str = "messages";

    pub fn new(pool: DbPool) -> Result<Self, MessageError> {
        let conn = pool.get()?;
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
					id TEXT PRIMARY KEY,
					text TEXT NOT NULL,
					reasoning TEXT,
					sender TEXT NOT NULL,
					timestamp INTEGER NOT NULL,
					tokens INTEGER,
					embedding BLOB
				)",
                Self::TABLE_NAME
            ),
            [],
        )?;

        Ok(Self { pool })
    }

    pub fn add(&mut self, id: &str, text: &str, reasoning: Option<&str>, sender: &str, tokens: Option<i32>, embedding: Option<Vec<u8>>) -> Result<(), MessageError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let conn = self.pool.get()?;
        conn.execute(
            &format!(
                "INSERT INTO {} (id, text, reasoning, sender, timestamp, tokens, embedding) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                Self::TABLE_NAME
            ),
            params![id, text, reasoning, sender, timestamp, tokens, embedding],
        )?;
        Ok(())
    }

    pub fn add_batch(&mut self, messages: &[(&str, &str, Option<&str>, &str, Option<i32>, Option<Vec<u8>>)]) -> Result<(), MessageError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(&format!(
                "INSERT INTO {} (id, text, reasoning, sender, timestamp, tokens, embedding) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                Self::TABLE_NAME
            ))?;

            for (id, text, reasoning, sender, tokens, embedding) in messages {
                stmt.execute(params![id, text, reasoning, sender, timestamp, tokens, embedding])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get(&mut self, id: &str) -> Result<Message, MessageError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(&format!(
            "SELECT id, text, reasoning, sender, timestamp, tokens, embedding FROM {} WHERE id = ?1",
            Self::TABLE_NAME
        ))?;

        let row = stmt.query_row(params![id], |row| {
			let sender_str: String = row.get(3)?;
            let sender = MessageRole::try_from(sender_str)
                .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
            Ok(Message {
                id: row.get(0)?,
                text: row.get(1)?,
                reasoning: row.get(2)?,
                sender,
                timestamp: row.get(4)?,
                tokens: row.get(5)?,
                embedding: row.get(6)?,
            })
        })?;
        Ok(row)
    }

    pub fn list(&mut self, limit: i64, offset: i64) -> Result<Vec<Message>, MessageError> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(&format!(
            "SELECT id, text, reasoning, sender, timestamp, tokens, embedding FROM {} ORDER BY timestamp DESC LIMIT ?1 OFFSET ?2",
            Self::TABLE_NAME
        ))?;

        let messages = stmt
            .query_map(params![limit, offset], |row| {
				let sender_str: String = row.get(3)?;
				let sender = MessageRole::try_from(sender_str)
					.map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
                Ok(Message {
                    id: row.get(0)?,
                    text: row.get(1)?,
                    reasoning: row.get(2)?,
                    sender,
                    timestamp: row.get(4)?,
                    tokens: row.get(5)?,
                    embedding: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()
            .map_err(MessageError::from)?;

        Ok(messages)
    }

    pub fn update_text(&mut self, id: &str, text: &str) -> Result<(), MessageError> {
        let conn = self.pool.get()?;
        conn.execute(
            &format!(
                "UPDATE {} SET text = ?2 WHERE id = ?1",
                Self::TABLE_NAME
            ),
            params![id, text],
        )?;
        Ok(())
    }

	pub fn update_reasoning(&mut self, id: &str, reasoning: &str) -> Result<(), MessageError> {
        let conn = self.pool.get()?;
        conn.execute(
            &format!(
                "UPDATE {} SET reasoning = ?2 WHERE id = ?1",
                Self::TABLE_NAME
            ),
            params![id, reasoning],
        )?;
        Ok(())
    }

    pub fn update_sender(&mut self, id: &str, sender: MessageRole) -> Result<(), MessageError> {
        let conn = self.pool.get()?;
        conn.execute(
            &format!(
                "UPDATE {} SET sender = ?2 WHERE id = ?1",
                Self::TABLE_NAME
            ),
            params![id, sender.to_string()],
        )?;
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> Result<(), MessageError> {
        let conn = self.pool.get()?;
        conn.execute(
            &format!(
                "DELETE FROM {} WHERE id = ?1",
                Self::TABLE_NAME
            ),
            params![id],
        )?;
        Ok(())
    }

    pub fn delete_batch(&mut self, ids: &[&str]) -> Result<(), MessageError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(&format!(
                "DELETE FROM {} WHERE id = ?1",
                Self::TABLE_NAME
            ))?;

            for id in ids {
                stmt.execute(params![id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

}

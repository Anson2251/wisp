use rusqlite::{params, Connection, Statement};
use thiserror::Error;

use super::statement::leak_stmt;

#[derive(Debug, Error)]
pub enum ConversationError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

pub struct Conversation {
    id: String,
    name: String,
    description: Option<String>,
    entry_message_id: String,
}

pub struct Conversations {
    conn: Connection,
	list_stmt: Statement<'static>,
	create_stmt: Statement<'static>,
	update_name_stmt: Statement<'static>,
	update_description_stmt: Statement<'static>,
	update_entry_message_id_stmt: Statement<'static>,
	get_stmt: Statement<'static>,
	delete_stmt: Statement<'static>,
}

impl Conversations {
    pub const TABLE_NAME: &'static str = "conversations";

    pub fn new(db_path: &str, message_table_name: &str) -> Result<Self, ConversationError> {
        let conn = Connection::open(db_path)?;

		conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
				id TEXT PRIMARY KEY,
				name TEXT NOT NULL,
				description TEXT,
				entry_message_id TEXT,
				FOREIGN KEY (entry_message_id) REFERENCES {} (id)
			)",
                Self::TABLE_NAME,
				message_table_name
            ),
            [],
        )?;

		unsafe {
			let create_stmt = leak_stmt(conn.prepare(&format!(
					"INSERT INTO {} (id, name, description, entry_message_id) VALUES (?1, ?2, ?3, ?4)",
					Self::TABLE_NAME
				))?
			);

			let update_name_stmt = leak_stmt(conn.prepare(&format!(
					"UPDATE {} SET name = ?2 WHERE id = ?1",
					Self::TABLE_NAME
				))?
			);

			let update_description_stmt = leak_stmt(conn.prepare(&format!(
					"UPDATE {} SET description = ?2 WHERE id = ?1",
					Self::TABLE_NAME
				))?
			);

			let update_entry_message_id_stmt = leak_stmt(conn.prepare(&format!(
					"UPDATE {} SET entry_message_id = ?2 WHERE id = ?1",
					Self::TABLE_NAME
				))?
			);

			let get_stmt = leak_stmt(conn.prepare(&format!(
					"SELECT * FROM {} WHERE id = ?1",
					Self::TABLE_NAME
				))?
			);

			let delete_stmt = leak_stmt(conn.prepare(&format!(
					"DELETE FROM {} WHERE id = ?1",
					Self::TABLE_NAME
				))?
			);

			let list_stmt = leak_stmt(conn.prepare(&format!(
					"SELECT * FROM {} ORDER BY name ASC LIMIT ?1 OFFSET ?2",
					Self::TABLE_NAME
				))?
			);

			Ok(Self {
				conn,
				list_stmt,
				update_name_stmt,
				update_description_stmt,
				update_entry_message_id_stmt,
				get_stmt,
				delete_stmt,
				create_stmt
			 })
		}
    }

	pub fn create(&mut self, id: &str, name: &str, description: &str, entry_message_id: &str) -> Result<(), ConversationError> {
		self.create_stmt
			.execute(params![id, name, description, entry_message_id])?;

		Ok(())
	}

	pub fn get(&mut self, id: &str) -> Result<Option<Conversation>, ConversationError> {
		let row = self.get_stmt
			.query_row(params![id], |row| {
				Ok(Conversation {
					id: row.get(0)?,
					name: row.get(1)?,
					description: row.get(2)?,
					entry_message_id: row.get(3)?,
				})
			})?;
		Ok(Some(row))
	}

	pub fn update_name(&mut self, id: &str, name: &str) -> Result<(), ConversationError> {
		self.update_name_stmt
			.execute(params![id, name])?;

		Ok(())
	}

	pub fn update_description(&mut self, id: &str, description: &str) -> Result<(), ConversationError> {
		self.update_description_stmt
			.execute(params![id, description])?;
		Ok(())
	}

	pub fn update_entry_message_id(&mut self, id: &str, entry_message_id: &str) -> Result<(), ConversationError> {
		self.update_entry_message_id_stmt
			.execute(params![id, entry_message_id])?;
		Ok(())
	}

	pub fn delete(&mut self, id: &str) -> Result<(), ConversationError> {
		self.delete_stmt
			.execute(params![id])?;
		Ok(())
	}

	pub fn list(&mut self) -> Result<Vec<Conversation>, ConversationError> {
		let conversations = self.list_stmt
			.query_map(params![], |row| {
				Ok(Conversation {
					id: row.get(0)?,
					name: row.get(1)?,
					description: row.get(2)?,
					entry_message_id: row.get(3)?,
				})
			})?
			.collect::<Result<Vec<_>, rusqlite::Error>>()
			.map_err(ConversationError::from)?;

		Ok(conversations)
	}

    pub fn conn(&mut self) -> &mut Connection {
        &mut self.conn
    }

}

use rusqlite::{params, Statement};
use super::DbPool;
use thiserror::Error;
use super::statement::leak_stmt;

#[derive(Debug, Error)]
pub enum ThreadError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("Invalid thread relation")]
    InvalidRelation,
	#[error("Invalid thread relation in batch operation at index {0}")]
	InvalidRelationBatch(usize),
}

pub struct Threads {
    pool: DbPool,
    add_stmt: Statement<'static>,
    delete_stmt: Statement<'static>,
    delete_with_parent_stmt: Statement<'static>,
	delete_with_child_stmt: Statement<'static>,
    get_children_stmt: Statement<'static>,
    get_parent_stmt: Statement<'static>,
    exists_stmt: Statement<'static>,
	exists_node_stmt: Statement<'static>,
	update_parent_stmt: Statement<'static>,
}

impl Threads {
    pub const TABLE_NAME: &'static str = "threads";

	/// Create a new `Threads` instance.
    pub fn new(pool: DbPool, message_table_name: &str, message_primary_key: &str) -> Result<Self, ThreadError> {
        let conn = pool.get().map_err(ThreadError::Pool)?;
        conn.execute(
            &format!("CREATE TABLE IF NOT EXISTS {} (
                id TEXT NOT NULL,
                parent_id TEXT NOT NULL,
                PRIMARY KEY (id, parent_id),
                FOREIGN KEY (id) REFERENCES {}({}) ON DELETE CASCADE,
                FOREIGN KEY (parent_id) REFERENCES {}({}) ON DELETE CASCADE
            )", Self::TABLE_NAME, message_table_name, message_primary_key,
               message_table_name, message_primary_key),
            [],
        )?;

        unsafe {
            let add_stmt = leak_stmt(conn.prepare(&format!(
                "INSERT INTO {} (id, parent_id) VALUES (?1, ?2)",
                Self::TABLE_NAME
            ))?);

            let delete_stmt = leak_stmt(conn.prepare(&format!(
                "DELETE FROM {} WHERE id = ?1 AND parent_id = ?2",
                Self::TABLE_NAME
            ))?);

            let delete_with_parent_stmt = leak_stmt(conn.prepare(&format!(
                "DELETE FROM {} WHERE parent_id = ?1",
                Self::TABLE_NAME
            ))?);

			let delete_with_child_stmt = leak_stmt(conn.prepare(&format!(
                "DELETE FROM {} WHERE id = ?1",
                Self::TABLE_NAME
            ))?);

            let get_children_stmt = leak_stmt(conn.prepare(&format!(
                "SELECT id FROM {} WHERE parent_id = ?1",
                Self::TABLE_NAME
            ))?);

            let get_parent_stmt = leak_stmt(conn.prepare(&format!(
                "SELECT parent_id FROM {} WHERE id = ?1",
                Self::TABLE_NAME
            ))?);

            let exists_stmt = leak_stmt(conn.prepare(&format!(
                "SELECT EXISTS(SELECT 1 FROM {} WHERE id = ?1 AND parent_id = ?2)",
                Self::TABLE_NAME
            ))?);

			let exists_node_stmt = leak_stmt(conn.prepare(&format!(
                "SELECT EXISTS(SELECT 1 FROM {} WHERE id = ?1)",
                Self::TABLE_NAME
            ))?);

			let update_parent_stmt = leak_stmt(conn.prepare(&format!(
                "UPDATE {} SET parent_id = ?1 WHERE id = ?2",
                Self::TABLE_NAME
            ))?);

            Ok(Self {
                pool,
                add_stmt,
                delete_stmt,
                delete_with_parent_stmt,
				delete_with_child_stmt,
                get_children_stmt,
                get_parent_stmt,
                exists_stmt,
				exists_node_stmt,
                update_parent_stmt,
            })
        }
    }

	/// Add a new thread relation.
    pub fn add(&mut self, message_id: &str, parent_id: &str) -> Result<(), ThreadError> {
        self.add_stmt.execute(params![message_id, parent_id])?;
        Ok(())
    }

	/// Add multiple thread relations in a batch.
    pub fn add_batch(&mut self, relations: &[(&str, &str)]) -> Result<(), ThreadError> {
        let mut conn = self.pool.get().map_err(ThreadError::Pool)?;
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(&format!(
                "INSERT INTO {} (id, parent_id) VALUES (?1, ?2)",
                Self::TABLE_NAME
            ))?;

            for (message_id, parent_id) in relations {
                stmt.execute(params![message_id, parent_id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

	/// Update the parent of a thread relation.
	pub fn update_parent(&mut self, message_id: &str, new_parent_id: &str) -> Result<(), ThreadError> {
		self.update_parent_stmt.execute(params![new_parent_id, message_id])?;
		Ok(())
	}

	/// Delete a thread relation.
    pub fn delete(&mut self, message_id: &str, parent_id: &str) -> Result<(), ThreadError> {
        if !self.exists(message_id, parent_id)? {
            return Err(ThreadError::InvalidRelation);
        }
        self.delete_stmt.execute(params![message_id, parent_id])?;
        Ok(())
    }

	/// Delete multiple thread relations in a batch.
    pub fn delete_batch(&mut self, relations: &[(&str, &str)]) -> Result<(), ThreadError> {
        // Check all relations exist first
        for (i, (message_id, parent_id)) in relations.iter().enumerate() {
            if !self.exists(message_id, parent_id)? {
                return Err(ThreadError::InvalidRelationBatch(i));
            }
        }

        // Now perform the deletions in a transaction
        let mut conn = self.pool.get().map_err(ThreadError::Pool)?;
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(&format!(
                "DELETE FROM {} WHERE id = ?1 AND parent_id = ?2",
                Self::TABLE_NAME
            ))?;

            for (message_id, parent_id) in relations {
                stmt.execute(params![message_id, parent_id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

	/// Delete all thread relations with a specific parent.
    pub fn delete_with_parent(&mut self, parent_id: &str) -> Result<(), ThreadError> {
        self.delete_with_parent_stmt.execute(params![parent_id])?;
        Ok(())
    }

	/// Delete all thread relation with a specific child.
	pub fn delete_with_child(&mut self, message_id: &str) -> Result<(), ThreadError> {
        self.delete_with_child_stmt.execute(params![message_id])?;
        Ok(())
    }

	/// Get all children of a specific parent.
    pub fn get_children(&mut self, parent_id: &str) -> Result<Vec<String>, ThreadError> {
        let mut children = Vec::new();
        for row in self.get_children_stmt.query_map(params![parent_id], |row| {
            row.get::<_, String>(0)
        })? {
            children.push(row?);
        }
        Ok(children)
    }

	/// Get the parent of a specific message.
    pub fn get_parent(&mut self, message_id: &str) -> Result<Option<String>, ThreadError> {
        let parent_id: Option<String> = self.get_parent_stmt.query_row(params![message_id], |row| {
            row.get::<_, Option<String>>(0)
        })?;
        Ok(parent_id)
    }

	/// Check if a thread relation exists with a specific message and parent.
    pub fn exists(&mut self, message_id: &str, parent_id: &str) -> Result<bool, ThreadError> {
        let exists = self.exists_stmt.query_row(params![message_id, parent_id], |row| {
            row.get::<_, bool>(0)
        })?;
        Ok(exists)
    }

	/// Check if a specific message exists in the thread.
    pub fn exists_node(&mut self, message_id: &str) -> Result<bool, ThreadError> {
        let exists = self.exists_node_stmt.query_row(params![message_id], |row| {
            row.get::<_, bool>(0)
        })?;
        Ok(exists)
    }

}

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiagramCacheEntry {
    pub svg: String,
    pub height: u32,
    pub width: u32,
}

pub struct DiagramCache {
    conn: Connection,
}

impl DiagramCache {
    pub fn new() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS diagram_cache (
                hash TEXT PRIMARY KEY,
                diagram_data TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }

    pub fn get(&self, hash: &str) -> Result<DiagramCacheEntry, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT diagram_data FROM diagram_cache WHERE hash = ?")
            .map_err(|e| e.to_string())?;

        let result: String = stmt
            .query_row(params![hash], |row| row.get(0))
            .map_err(|_| "Cache miss".to_string())?;

        serde_json::from_str(&result).map_err(|e| e.to_string())
    }

    pub fn put(&self, hash: &str, entry: &DiagramCacheEntry) {
        let data = serde_json::to_string(entry).expect("Failed to serialize diagram data");
        self.conn
            .execute(
                "INSERT OR REPLACE INTO diagram_cache (hash, diagram_data) VALUES (?1, ?2)",
                params![hash, data],
            )
            .expect("Failed to insert into cache");
    }

    pub fn clear(&self) {
        self.conn
            .execute("DELETE FROM diagram_cache", [])
            .expect("Failed to clear cache");
    }
}

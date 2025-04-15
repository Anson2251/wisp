use rusqlite::{params, Connection};

pub struct AstCache {
    conn: Connection,
}

#[allow(unused)]
impl AstCache {
    pub fn new() -> Self {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory SQLite DB");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ast_cache (
                hash TEXT PRIMARY KEY,
                ast_json TEXT NOT NULL,
                rendered_html TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create cache table");
        Self { conn }
    }

    pub fn get(&self, hash: &str) -> Option<String> {
        self.conn
            .query_row(
                "SELECT rendered_html FROM ast_cache WHERE hash = ?",
                [hash],
                |row| row.get(0),
            )
            .ok()
    }

    pub fn put(&self, hash: &str, ast_json: &str, rendered_html: &str) {
        self.conn.execute(
            "INSERT OR REPLACE INTO ast_cache (hash, ast_json, rendered_html) VALUES (?1, ?2, ?3)",
            params![hash, ast_json, rendered_html],
        ).expect("Failed to insert into cache");
    }
}

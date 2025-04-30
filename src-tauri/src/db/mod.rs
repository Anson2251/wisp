use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

pub mod threads;
pub mod messages;
pub mod chat;
pub mod conversations;
pub mod types;

pub type DbPool = Arc<Pool<SqliteConnectionManager>>;

pub fn create_pool(db_path: &str) -> DbPool {
    let manager = SqliteConnectionManager::file(db_path)
        .with_init(|conn| {
            conn.pragma_update(None, "foreign_keys", "ON")?;
            Ok(())
        });
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create connection pool");
    Arc::new(pool)
}

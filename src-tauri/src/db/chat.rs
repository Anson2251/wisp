use tauri::{AppHandle, Manager};
use std::path::PathBuf;

use super::messages::{Messages, MessageError};
use super::threads::{Threads, ThreadError};
use super::conversations::{Conversations, ConversationError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Invalid chat relation")]
    Message(#[from] MessageError),
    #[error("Invalid thread relation")]
    Thread(#[from] ThreadError),
	#[error("Invalid conversation relation")]
	Conversation(#[from] ConversationError),
}

struct Chat {
	app_handle: AppHandle,
	thread_manager: Threads,
	conversation_manager: Conversations,
	messages_manager: Messages,
}

impl Chat {
    pub fn new(app_handle: &AppHandle) -> Result<Self, ChatError> {
		let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir");
        println!("App dir: {:?}", app_dir);
        std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
		let db_path = PathBuf::from(app_dir).join("messages.db");
        let db_path = db_path.to_str().expect("Failed to reach database path");

		let messages_manager = Messages::new(&db_path)?;
		let thread_manager = Threads::new(&db_path, "messages", "id")?;
        let conversation_manager = Conversations::new(&db_path, "messages")?;

        Ok(Chat {
			app_handle: app_handle.clone(),
			thread_manager,
			conversation_manager,
			messages_manager,
		})
    }
}

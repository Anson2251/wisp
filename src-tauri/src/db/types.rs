use thiserror::Error;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageNode {
    pub message_id: String,
	pub parent_id: Option<String>,
    pub children: Vec<MessageNode>,
}


#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("Chat Message error: {0}")]
    Message(#[from] MessageError),
    #[error("Chat Thread error: {0}")]
    Thread(#[from] ThreadError),
	#[error("Chat Conversation error: {0}")]
	Conversation(#[from] ConversationError),
}


#[derive(Debug, Error)]
pub enum ConversationError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
	#[error("Invalid operation: {0}")]
	InvalidOperation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub entry_message_id: Option<String>,
}


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


#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("Invalid message role: {0}")]
    InvalidRole(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

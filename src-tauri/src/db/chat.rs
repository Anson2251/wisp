use super::conversations::Conversations;
use super::messages::Messages;
use super::threads::Threads;
use super::types::{ChatError, Conversation, ConversationError, Message};
use super::{create_pool, DbPool};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub struct Chat {
    pool: DbPool,
    pub thread_manager: Threads,
    pub conversation_manager: Conversations,
    pub messages_manager: Messages,
}

#[allow(unused)]
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

        let pool = create_pool(db_path);
        let messages_manager = Messages::new(pool.clone())?;
        let thread_manager = Threads::new(pool.clone(), "messages", "id")?;
        let conversation_manager = Conversations::new(pool.clone(), "messages")?;

        Ok(Chat {
            pool,
            thread_manager,
            conversation_manager,
            messages_manager,
        })
    }

    /// Creates a new conversation with initial system message
    pub fn create_conversation(
        &mut self,
        conversation_id: &str,
        name: &str,
        description: &str,
    ) -> Result<(), ChatError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        self.conversation_manager.create(
            conversation_id,
            name,
            Some(description),
            None,
        )?;

        tx.commit()?;
        Ok(())
    }

    /// Adds a message to an existing conversation thread
    pub fn add_message(
        &mut self,
        conversation_id: &str,
        message_id: &str,
        text: &str,
        sender: &str,
        parent_message_id: Option<&str>,
    ) -> Result<(), ChatError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        // Add the message
        self.messages_manager
            .add(message_id, text, sender, None, None)?;

        // Link to parent message
        self.thread_manager.add(message_id, parent_message_id)?;

        // Link to conversation's entry message if no parent specified
        if parent_message_id.is_none() {
            let conv =
                self.conversation_manager
                    .get(conversation_id)?
                    .ok_or(ChatError::Conversation(ConversationError::Database(
                        rusqlite::Error::QueryReturnedNoRows,
                    )))?;
            let conv_id: &str = &conv.id;
            self.conversation_manager.update_entry_message_id(conv_id, message_id)?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Gets full message thread for a conversation
    pub fn get_conversation_thread(
        &mut self,
        conversation_id: &str,
    ) -> Result<Vec<Message>, ChatError> {
        let conv =
            self.conversation_manager
                .get(conversation_id)?
                .ok_or(ChatError::Conversation(ConversationError::Database(
                    rusqlite::Error::QueryReturnedNoRows,
                )))?;

		if conv.entry_message_id.is_none() { return Ok(vec![]) }

        let entry_id = conv
            .entry_message_id
            .as_deref()
            .ok_or(ChatError::Conversation(ConversationError::Database(
                rusqlite::Error::InvalidQuery,
            )))?;

        // Start with the entry message
        let mut messages = vec![self.messages_manager.get(entry_id)?];

        // Recursively get all threaded messages
        let mut current_level = vec![entry_id.to_string()];
        while !current_level.is_empty() {
            let mut next_level = Vec::new();
            for parent_id in &current_level {
                let children = self.thread_manager.get_children(parent_id)?;
                for child_id in children {
                    messages.push(self.messages_manager.get(&child_id)?);
                    next_level.push(child_id);
                }
            }
            current_level = next_level;
        }

        Ok(messages)
    }

    /// Deletes a conversation and all its messages
    pub fn delete_conversation(&mut self, conversation_id: &str) -> Result<(), ChatError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        // Get all message IDs in conversation
        let messages = self.get_conversation_thread(conversation_id)?;

        // Delete all messages
        for message in messages {
            self.messages_manager.delete(&message.id)?;
        }

        // Delete conversation
        self.conversation_manager.delete(conversation_id)?;

        tx.commit()?;
        Ok(())
    }

    /// Lists all conversations with their names
    pub fn list_conversations(&mut self) -> Result<Vec<Conversation>, ChatError> {
        let convs = self.conversation_manager.list()?;
        Ok(convs)
    }

    /// Updates a message's content
    pub fn update_message(&mut self, message_id: &str, new_text: &str) -> Result<(), ChatError> {
        self.messages_manager.update_text(message_id, new_text)?;
        Ok(())
    }

    /// Deletes a message and its thread relationships, returns the new parent message ID if any.
    pub fn delete_message(&mut self, message_id: &str, recursive: bool) -> Result<Option<String>, ChatError> {
        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;

        if recursive {
            // Delete all children of the message recursively
            let mut all_children = Vec::new();
            let mut current_children = self.thread_manager.get_children(message_id)?;
            while !current_children.is_empty() {
                let mut next_children = Vec::new();
                for child in &current_children {
                    next_children.extend(self.thread_manager.get_children(&child)?);
                }
                all_children.extend(current_children);
                current_children = next_children;
            }

            // Delete all children
            for child_id in all_children {
                self.messages_manager.delete(&child_id)?;
            }

            // Delete the original message
            self.messages_manager.delete(message_id)?;

			tx.commit()?;
        	Ok(None)
        } else {
            let parent = self.thread_manager.get_parent(message_id)?;
            let children = self.thread_manager.get_children(message_id)?;

            // Update parent of children to the parent of the deleted message, or None if no parent exists
            for child in children {
                self.thread_manager.update_parent(&child, parent.clone().as_deref());
            }

            // Delete message
            self.messages_manager.delete(message_id)?;
			tx.commit()?;
        	Ok(parent)
        }
    }
}

use std::sync::Mutex;

use crate::{
    cache::AstCache,
    db::types::{Conversation, Message},
    utils::compute_content_hash,
};
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::types::AppData;
use crate::utils::get_uuid_v4;

// Content utilities
#[tauri::command]
pub fn hash_content(content: String) -> String {
    compute_content_hash(&content)
}

#[tauri::command]
pub async fn put_cached_render(hash: String, ast_json: String, rendered_html: String) {
    AstCache::new().put(&hash, &ast_json, &rendered_html);
}

// OpenAI integration
#[tauri::command]
pub async fn ask_openai_stream(app_handle: AppHandle, messages: Vec<Value>) -> Result<(), String> {
    crate::api::ask_openai_stream(app_handle, messages).await
}

// Chat operations
#[tauri::command]
pub async fn create_conversation(
    app_handle: AppHandle,
    name: String,
    description: String
) -> Result<String, String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();

    let id = get_uuid_v4();
    state.chat.create_conversation(
        &id,
        &name,
        &description,
    ).map(|_| id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_message(
    app_handle: AppHandle,
    conversation_id: String,
    text: String,
    sender: String,
    parent_id: Option<String>
) -> Result<String, String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();

    let message_id = get_uuid_v4();
    state.chat.add_message(
        &conversation_id,
        &message_id,
        &text,
        &sender,
        parent_id.as_deref()
    ).map(|_| message_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_message(
	app_handle: AppHandle,
    message_id: String,
    text: String
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();
    state.chat.update_message(
        &message_id,
        &text
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_message(
    app_handle: AppHandle,
    message_id: String,
	recursive: bool
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();
    state.chat.delete_message(
        &message_id,
		recursive
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversation_thread(
    app_handle: AppHandle,
    conversation_id: String
) -> Result<Vec<Message>, String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();

    state.chat.get_conversation_thread(&conversation_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_conversation_entry_id(
	app_handle: AppHandle,
    conversation_id: String,
	message_id: String,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();
    state.chat.conversation_manager.update_entry_message_id(&conversation_id, &message_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(
    app_handle: AppHandle,
    conversation_id: String
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();

    state.chat.delete_conversation(&conversation_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_conversations(
    app_handle: AppHandle
) -> Result<Vec<Conversation>, String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();

    state.chat.list_conversations().map_err(|e| e.to_string())
}

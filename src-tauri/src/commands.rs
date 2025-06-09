use std::sync::Mutex;
use std::collections::HashMap;

use crate::{
    cache::DiagramCacheEntry,
    db::types::{Conversation, Message, ThreadTreeItem},
    utils::compute_content_hash,
    inet::HttpClient,
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
pub async fn get_cached_diagram(app_handle: AppHandle, hash: String) -> Option<DiagramCacheEntry> {
    let state = app_handle.state::<Mutex<AppData>>();
    let state = state.lock().unwrap();

    state.diagram_cache.get(&hash).ok()
}

#[tauri::command]
pub async fn put_cached_diagram(app_handle: AppHandle, hash: String, entry: DiagramCacheEntry) {
    let state = app_handle.state::<Mutex<AppData>>();
    let state = state.lock().unwrap();

    state.diagram_cache.put(&hash, &entry);
}

#[tauri::command]
pub async fn clear_diagram_cache(app_handle: AppHandle) {
    let state = app_handle.state::<Mutex<AppData>>();
    let state = state.lock().unwrap();

    state.diagram_cache.clear();
}

#[tauri::command]
pub async fn get_url(
    url: String,
    headers: Option<HashMap<String, String>>,
    parse_json: bool
) -> Result<serde_json::Value, String> {
    let client = HttpClient::new();
    client.get(url, headers, parse_json).await
}

#[tauri::command]
pub async fn post_url(
    url: String,
    body: String,
    headers: Option<HashMap<String, String>>,
    parse_json: bool
) -> Result<serde_json::Value, String> {
    let client = HttpClient::new();
    client.post(url, body, headers, parse_json).await
}

// API Key Management
#[tauri::command]
pub fn set_api_key(
    app_handle: AppHandle,
	name: String,
    key: String
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let state = state.lock().unwrap();
    state.key_manager.set_api_key(&name, &key)
}

#[tauri::command]
pub fn get_api_key(
    app_handle: AppHandle,
	name: String
) -> Result<String, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let state = state.lock().unwrap();
    state.key_manager.get_api_key(&name)
}

#[tauri::command]
pub fn delete_api_key(
    app_handle: AppHandle,
    name: String
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
	let state = state.lock().unwrap();
    state.key_manager.delete_api_key(&name)
}

// OpenAI integration
#[tauri::command]
pub async fn ask_openai_stream(app_handle: AppHandle, messages: Vec<Value>) -> Result<(), String> {
	crate::api::ask_openai_stream(app_handle, messages).await.map_err(|e| e.to_string())
}

// Chat operations
#[tauri::command]
pub async fn create_conversation(
    app_handle: AppHandle,
    name: String,
    description: String,
) -> Result<String, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();

    let id = get_uuid_v4();
    state
        .chat
        .create_conversation(&id, &name, &description)
        .map(|_| id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_message(
    app_handle: AppHandle,
    conversation_id: String,
    text: String,
	reasoning: Option<String>,
    sender: String,
    parent_id: Option<String>,
) -> Result<String, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();

    let message_id = get_uuid_v4();
    state
        .chat
        .add_message(
            &conversation_id,
            &message_id,
            &text,
			reasoning.as_deref(),
            &sender,
            parent_id.as_deref(),
        )
        .map(|_| message_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_message(
    app_handle: AppHandle,
    message_id: String,
    text: String,
	reasoning: Option<String>,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();
    state
        .chat
        .update_message(&message_id, &text)
        .map_err(|e| e.to_string())?;
	if let Some(reasoning) = reasoning {
        let _ = state.chat.messages_manager.update_reasoning(&message_id, &reasoning);
	}
	Ok(())
}

#[tauri::command]
pub async fn get_message(
    app_handle: AppHandle,
    message_id: String,
) -> Result<Message, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();
    state
        .chat
        .messages_manager.get(&message_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_message(
    app_handle: AppHandle,
    message_id: String,
    recursive: bool,
) -> Result<Option<String>, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();
    state
        .chat
        .delete_message(&message_id, recursive)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_message_involved(
    app_handle: AppHandle,
    conversation_id: String,
) -> Result<Vec<Message>, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();

    state
        .chat
        .get_all_message_involved(&conversation_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_thread_tree(
	app_handle: AppHandle,
	conversation_id: String,
) -> Result<Vec<ThreadTreeItem>, String> {
	let state = app_handle.state::<Mutex<AppData>>();
	let mut state = state.lock().unwrap();

	state
		.chat
		.get_thread_tree(&conversation_id)
		.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_conversation_entry_id(
    app_handle: AppHandle,
    conversation_id: String,
    message_id: String,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();
    state
        .chat
        .conversation_manager
        .update_entry_message_id(&conversation_id, Some(&message_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(
    app_handle: AppHandle,
    conversation_id: String,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();

    state
        .chat
        .delete_conversation(&conversation_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_conversation(
	app_handle: AppHandle,
	conversation_id: String,
	name: Option<String>,
	description: Option<String>,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();

	if let Some(name) = name {
		state.chat.conversation_manager.update_name(&conversation_id, &name).map_err(|e| e.to_string())?;
	}

	if let Some(description) = description {
		state.chat.conversation_manager.update_description(&conversation_id, &description).map_err(|e| e.to_string())?;
	}
    Ok(())
}

#[tauri::command]
pub async fn list_conversations(app_handle: AppHandle) -> Result<Vec<Conversation>, String> {
    let state = app_handle.state::<Mutex<AppData>>();
    let mut state = state.lock().unwrap();

    state.chat.list_conversations().map_err(|e| e.to_string())
}

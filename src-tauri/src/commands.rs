use crate::{
    cache::AstCache,
    // db::{Database, Message},
    utils::compute_content_hash,
};
use serde_json::Value;
use tauri::AppHandle;

#[tauri::command]
pub fn hash_content(content: String) -> String {
    compute_content_hash(&content)
}

#[tauri::command]
pub async fn put_cached_render(hash: String, ast_json: String, rendered_html: String) {
    let cache = AstCache::new();
    cache.put(&hash, &ast_json, &rendered_html);
}

#[tauri::command]
pub async fn ask_openai_stream(app_handle: AppHandle, messages: Vec<Value>) -> Result<(), String> {
    crate::api::ask_openai_stream(app_handle, messages).await
}

#[tauri::command]
pub async fn save_message(
    app_handle: AppHandle,
    id: String,
    text: String,
    sender: String,
) -> Result<(), String> {
    let db = Database::new(&app_handle).map_err(|e| e.to_string())?;
    db.save_message(&id, &text, &sender)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages(app_handle: AppHandle) -> Result<Vec<Message>, String> {
    let db = Database::new(&app_handle).map_err(|e| e.to_string())?;
    db.get_messages().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clear_messages(app_handle: AppHandle) -> Result<(), String> {
    let db = Database::new(&app_handle).map_err(|e| e.to_string())?;
    db.clear_messages().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_message(app_handle: AppHandle, id: String) -> Result<(), String> {
    let db = Database::new(&app_handle).map_err(|e| e.to_string())?;
    db.delete_message(&id).map_err(|e| e.to_string())
}

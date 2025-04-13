use crate::{cache::AstCache, utils::compute_content_hash};
use serde_json::Value;
use tauri::AppHandle;

#[tauri::command]
pub fn hash_content(content: String) -> String {
    compute_content_hash(&content)
}

#[tauri::command]
pub async fn get_cached_render(hash: String) -> Option<String> {
    let cache = AstCache::new();
    cache.get(&hash)
}

#[tauri::command]
pub async fn put_cached_render(hash: String, ast_json: String, rendered_html: String) {
    let cache = AstCache::new();
    cache.put(&hash, &ast_json, &rendered_html);
}

#[tauri::command]
pub async fn ask_openai_stream(
    app_handle: AppHandle,
    messages: Vec<Value>,
) -> Result<(), String> {
    crate::api::ask_openai_stream(app_handle, messages).await
}

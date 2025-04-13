use futures::StreamExt;
use reqwest::header;
use rusqlite::{Connection, params};
use serde_json::json;
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Emitter};

struct AstCache {
    conn: Connection,
}

impl AstCache {
    fn new() -> Self {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory SQLite DB");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS ast_cache (
                hash TEXT PRIMARY KEY,
                ast_json TEXT NOT NULL,
                rendered_html TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create cache table");
        Self { conn }
    }

    fn get(&self, hash: &str) -> Option<String> {
        self.conn
            .query_row(
                "SELECT rendered_html FROM ast_cache WHERE hash = ?",
                [hash],
                |row| row.get(0),
            )
            .ok()
    }

    fn put(&self, hash: &str, ast_json: &str, rendered_html: &str) {
        self.conn.execute(
            "INSERT OR REPLACE INTO ast_cache (hash, ast_json, rendered_html) VALUES (?1, ?2, ?3)",
            params![hash, ast_json, rendered_html],
        ).expect("Failed to insert into cache");
    }
}

const URL_BASE: &str = "https://api.siliconflow.cn/v1";
const MODEL: &str = "Qwen/Qwen2.5-7B-Instruct";

fn compute_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
fn hash_content(content: String) -> String {
    compute_content_hash(&content)
}

#[tauri::command]
async fn get_cached_render(hash: String) -> Option<String> {
    let cache = AstCache::new();
    cache.get(&hash)
}

#[tauri::command]
async fn put_cached_render(hash: String, ast_json: String, rendered_html: String) {
    let cache = AstCache::new();
    cache.put(&hash, &ast_json, &rendered_html);
}

#[tauri::command]
async fn ask_openai_stream(
    app_handle: AppHandle,
    messages: Vec<serde_json::Value>,
) -> Result<(), String> {
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set")?;
    let client = reqwest::Client::new();
    let mut stream = client
        .post(format!("{}/chat/completions", URL_BASE))
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "text/event-stream")
        .json(&json!({
            "model": MODEL,
            "messages": messages,
            "stream": true
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to send request to OpenAI: {}", e))?
        .bytes_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    if text.contains("data:") {
                        let data_part = text.splitn(2, "data:").nth(1).unwrap_or("");
                        if let Ok(json_value) =
                            serde_json::from_str::<serde_json::Value>(data_part.trim())
                        {
                            if let Some(content) = json_value
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                            {
                                if let Some(chunk_text) = content.as_str() {
                                    // Replace newlines with HTML line breaks for proper rendering
                                    let formatted_text = chunk_text;
                                    app_handle
                                        .emit("openai_stream_chunk", formatted_text)
                                        .map_err(|e| e.to_string())?;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading stream chunk: {}", e),
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ask_openai_stream,
            get_cached_render,
            hash_content,
            put_cached_render
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

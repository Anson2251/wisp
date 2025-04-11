use futures::StreamExt;
use reqwest::header;
use serde_json::json;
use tauri::{AppHandle, Emitter};

const URL_BASE: &str = "https://api.siliconflow.cn/v1";
const MODEL: &str = "Qwen/Qwen2.5-7B-Instruct";

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
        .invoke_handler(tauri::generate_handler![ask_openai_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

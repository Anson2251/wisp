use futures::StreamExt;
use reqwest::header;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

const URL_BASE: &str = "https://api.siliconflow.cn/v1";
const MODEL: &str = "Qwen/Qwen2.5-7B-Instruct";

pub async fn ask_openai_stream(app_handle: AppHandle, messages: Vec<Value>) -> Result<(), String> {
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
                        if let Ok(json_value) = serde_json::from_str::<Value>(data_part.trim()) {
                            if let Some(content) = json_value
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                            {
                                if let Some(chunk_text) = content.as_str() {
                                    app_handle
                                        .emit("openai_stream_chunk", chunk_text)
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

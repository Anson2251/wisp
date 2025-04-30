mod api;
mod cache;
mod commands;
mod db;
mod utils;
use tauri::{Builder, Manager};
use db::chat::Chat;
use std::sync::Mutex;
mod types;
use types::AppData;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
		.setup(|app| {
			app.manage(Mutex::new(AppData {
				chat: Chat::new(app.handle())?,
			}));
			Ok(())
		})
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::ask_openai_stream,
            // commands::get_cached_render,
            commands::hash_content,
            commands::put_cached_render,
            commands::create_conversation,
            commands::add_message,
            commands::get_conversation_thread,
            commands::delete_conversation,
            commands::list_conversations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

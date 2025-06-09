mod api;
mod cache;
mod commands;
mod db;
mod utils;
mod inet;
use tauri::{Builder, Manager};
use db::chat::Chat;
use cache::DiagramCache;
use std::sync::Mutex;
mod types;
use types::AppData;

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
		.setup(|app| {
			let window = app.get_webview_window("main").unwrap();
			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::Sidebar, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

			app.manage(Mutex::new(AppData {
				chat: Chat::new(app.handle())?,
				diagram_cache: DiagramCache::new()?,
			}));
			Ok(())
		})
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::ask_openai_stream,
            // commands::get_cached_render,
            commands::hash_content,
            commands::put_cached_diagram,
			commands::get_cached_diagram,
			commands::clear_diagram_cache,
            commands::create_conversation,
            commands::add_message,
			commands::get_message,
			commands::update_message,
			commands::delete_message,
            commands::get_all_message_involved,
			commands::get_thread_tree,
            commands::delete_conversation,
            commands::list_conversations,
			commands::update_conversation_entry_id,
			commands::update_conversation,
			commands::get_url,
			commands::post_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

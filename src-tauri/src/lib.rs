mod api;
mod cache;
mod commands;
mod configs;
mod db;
mod utils;
mod inet;
mod key_manager;
use tauri::{Builder, Manager};
use db::chat::Chat;
use cache::DiagramCache;
use key_manager::KeyManager;
use configs::ConfigManager;
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

			let config_manager = ConfigManager::new(app.handle())?;

			// set all fields of AppData to default values if they are None
			config_manager.save().expect("Failed to save config");

			app.manage(Mutex::new(AppData {
				chat: Chat::new(app.handle())?,
				diagram_cache: DiagramCache::new()?,
				key_manager: KeyManager::new("wisp".to_string()),
				config_manager,
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
			commands::set_api_key,
            commands::get_api_key,
            commands::delete_api_key,
			commands::configs_get_providers,
			commands::configs_get_provider,
			commands::configs_create_provider,
			commands::configs_update_provider,
			commands::configs_delete_provider,
			commands::configs_add_model,
			commands::configs_get_model,
			commands::configs_update_model,
			commands::configs_delete_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

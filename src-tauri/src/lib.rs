mod cache;
mod api;
mod utils;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::ask_openai_stream,
            commands::get_cached_render,
            commands::hash_content,
            commands::put_cached_render
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

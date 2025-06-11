use crate::db;
use db::chat::Chat;
use super::cache::DiagramCache;
use super::key_manager::KeyManager;
use super::configs::ConfigManager;

pub struct AppData {
	pub chat: Chat,
	pub diagram_cache: DiagramCache,
	pub key_manager: KeyManager,
	pub config_manager: ConfigManager,
}

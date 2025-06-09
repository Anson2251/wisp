use crate::db;
use db::chat::Chat;
use super::cache::DiagramCache;
use super::key_manager::KeyManager;

pub struct AppData {
	pub chat: Chat,
	pub diagram_cache: DiagramCache,
	pub key_manager: KeyManager,
}

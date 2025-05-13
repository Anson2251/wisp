use crate::db;
use db::chat::Chat;
use super::cache::DiagramCache;

pub struct AppData {
	pub chat: Chat,
	pub diagram_cache: DiagramCache,
}



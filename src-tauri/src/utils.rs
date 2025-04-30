use sha2::{Digest, Sha256};
use uuid::Uuid;

pub fn compute_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

pub fn get_uuid_v4() -> String {
    Uuid::new_v4().to_string().to_uppercase()
}

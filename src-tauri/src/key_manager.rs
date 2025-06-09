use keyring::Entry;

pub struct KeyManager {
    service_name: String,
}

impl KeyManager {
    pub fn new(service_name: String) -> Self {
        Self { service_name }
    }

	fn get_credential_name(&self, name: &str) -> String {
		format!("api_key.{}", name)
	}

    pub fn set_api_key(&self, name: &str, key: &str) -> Result<(), String> {
        Entry::new(&self.service_name, &self.get_credential_name(name))
            .map_err(|e| e.to_string())?
            .set_password(key)
            .map_err(|e| e.to_string())
    }

    pub fn get_api_key(&self, name: &str) -> Result<String, String> {
        Entry::new(&self.service_name, &self.get_credential_name(name))
            .map_err(|e| e.to_string())?
            .get_password()
            .map_err(|e| e.to_string())
    }

    pub fn delete_api_key(&self, name: &str) -> Result<(), String> {
        Entry::new(&self.service_name, &self.get_credential_name(name))
            .map_err(|e| e.to_string())?
            .delete_credential()
            .map_err(|e| e.to_string())
    }
}

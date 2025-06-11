pub mod provider;
pub mod model;

use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use toml;
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Config {
	providers: Vec<provider::Provider>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
	#[error("IO Error: {0}")]
	IoError(#[from] io::Error),
	#[error("TOML Deserialise Error: {0}")]
	TomlDeserialiseError(#[from] toml::de::Error),
	#[error("TOML Serialise Error: {0}")]
	TomlSerialiseError(#[from] toml::ser::Error),
}

pub struct ConfigManager {
    config_path: PathBuf,
    configs: Mutex<Config>,
}

#[allow(unused)]
impl ConfigManager {
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let config_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get config directory");

        let config_path = config_dir.join("configs.toml");

		let toml_content = if config_path.exists() {
            fs::read_to_string(&config_path).unwrap_or_default()
        } else {
            String::new()
        };

		let configs: Config = toml::from_str(&toml_content).unwrap_or_default();

        Ok(Self {
            config_path,
            configs: Mutex::new(configs)
        })
    }

    pub fn add_provider(&self, provider: provider::Provider) -> Result<(), ConfigError> {
        let mut configs = self.configs.lock().unwrap();
        configs.providers.push(provider);
		self.save()?;
		Ok(())
    }

    pub fn get_providers(&self) -> Vec<provider::Provider> {
        self.configs.lock().unwrap().providers.clone()
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_str = toml::to_string(&self.configs)?;
        fs::write(&self.config_path, config_str)?;
        Ok(())
    }

    pub fn get_provider(&self, name: &str) -> Option<provider::Provider> {
        let configs = self.configs.lock().unwrap();
        configs.providers.iter().find(|p| p.name == name).cloned()
    }

    pub fn update_provider(&self, name: &str, provider: provider::Provider) -> Result<(), ConfigError> {
        let mut configs = self.configs.lock().unwrap();
        if let Some(index) = configs.providers.iter().position(|p| p.name == name) {
            configs.providers[index] = provider;
            self.save()?;
            Ok(())
        } else {
            Err(ConfigError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "Provider not found",
            )))
        }
    }

    pub fn delete_provider(&self, name: &str) -> Result<(), ConfigError> {
        let mut configs = self.configs.lock().unwrap();
        if let Some(index) = configs.providers.iter().position(|p| p.name == name) {
            configs.providers.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(ConfigError::IoError(io::Error::new(
                io::ErrorKind::NotFound,
                "Provider not found",
            )))
        }
    }
}

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
	#[error("IO Error in ConfigManager: {0}")]
	IoError(#[from] io::Error),
	#[error("Config TOML Deserialise Error: {0}")]
	TomlDeserialiseError(#[from] toml::de::Error),
	#[error("Config TOML Serialise Error: {0}")]
	TomlSerialiseError(#[from] toml::ser::Error),
	#[error("Provider Not Found Error: {0}")]
	ProviderNotFoundError(String),
	#[error("Provider Already Exists Error: {0}")]
	ProviderAlreadyExistsError(String),
}

pub struct ConfigManager {
    config_path: PathBuf,
    configs: Mutex<Config>,
}

impl ConfigManager {
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let config_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get config directory");

        let config_path = config_dir.join("configs.toml");

		let toml_content = fs::read_to_string(&config_path).unwrap_or_default();

		let configs = toml::from_str::<Config>(&toml_content).unwrap_or_default();

        Ok(Self {
            config_path,
            configs: Mutex::new(configs)
        })
    }

	/// Add a new provider to the config. If the
	/// provider already exists, return ProviderAlreadyExistsError
    pub fn add_provider(&self, provider: provider::Provider) -> Result<(), ConfigError> {
		println!("Adding provider: {}", provider.name);
		if self.exists_provider(&provider.name) {
			println!("provider already exists");
			return Err(ConfigError::ProviderAlreadyExistsError(provider.name.clone()))
		}
		let mut configs = self.configs.lock().unwrap();
        configs.providers.push(provider);
		std::mem::drop(configs); // Explicitly drop the lock before saving
		println!("provider added successfully");
		self.save()?;
		println!("provider saved successfully");
		Ok(())
    }

	/// Check if a provider with the given name exists.
	pub fn exists_provider(&self, name: &str) -> bool {
		let configs = self.configs.lock().unwrap();
		configs.providers.iter().any(|p| p.name == name)
	}

	/// Get all providers.
    pub fn get_providers(&self) -> Vec<provider::Provider> {
        self.configs.lock().unwrap().providers.clone()
    }

	/// Save the current config to the file.
	///
	/// MUST UNLOCK THE MUTEX configs BEFORE CALLING THIS METHOD
    pub fn save(&self) -> Result<(), ConfigError> {
        let config_str = toml::to_string(&self.configs)?;
        fs::write(&self.config_path, config_str)?;
        Ok(())
    }

	/// Get a provider by name.
    pub fn get_provider(&self, name: &str) -> Option<provider::Provider> {
        let configs = self.configs.lock().unwrap();
        configs.providers.iter().find(|p| p.name == name).cloned()
    }

	/// Update a provider with the given name.
	/// If the provider does not exist, return ProviderNotFoundError.
    pub fn update_provider(&self, name: &str, provider: provider::Provider) -> Result<(), ConfigError> {
        let mut configs = self.configs.lock().unwrap();
        if let Some(index) = configs.providers.iter().position(|p| p.name == name) {
            configs.providers[index] = provider;
			std::mem::drop(configs);
            self.save()?;
            Ok(())
        } else {
            Err(ConfigError::ProviderNotFoundError(name.to_string()))
        }
    }

	/// Delete a provider by name.
	/// If the provider does not exist, return ProviderNotFoundError.
    pub fn delete_provider(&self, name: &str) -> Result<(), ConfigError> {
        let mut configs = self.configs.lock().unwrap();
        if let Some(index) = configs.providers.iter().position(|p| p.name == name) {
            configs.providers.remove(index);
			std::mem::drop(configs);
            self.save()?;
            Ok(())
        } else {
            Err(ConfigError::ProviderNotFoundError(name.to_string()))
        }
    }
}

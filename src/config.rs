extern crate tempfile;

use std::path::PathBuf;

// Add this line to import the crate
use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)] // Implement PartialEq trait
pub struct AppConfig {
    pub api_key: String,
    pub server_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: "default_api_key".to_string(),
            server_url: "https://example.com".to_string(),
        }
    }
}

pub fn read_config() -> Result<AppConfig, ConfyError> {
    confy::load("myapp", "AppConfig")
}

pub fn read_config_path(config_file_path: &PathBuf) -> Result<AppConfig, ConfyError> {
    // Create a new configuration instance with the specified file path
    let config = confy::load_path(config_file_path.clone())
        .or_else(|_| {
            // If the file doesn't exist or there's an error reading it, use the default configuration
            Ok(AppConfig::default())
        })?;

    Ok(config)
}

pub fn write_config_path(config: &AppConfig, config_file_path: &PathBuf) -> Result<(), ConfyError> {
    confy::store_path(config_file_path, config)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_read_default_config() {
        // Perform the test
        let config_result = read_config();

        // Assert that the result is the default configuration
        assert_eq!(
            config_result.unwrap(),
            AppConfig {
                api_key: "default_api_key".to_string(),
                server_url: "https://example.com".to_string(),
            }
        );
    }

    #[test]
    fn test_write_and_read_config() {
        const TMP_FILE: &str = "/tmp/myapp/config_file.toml";
        let config_file_path: PathBuf = TMP_FILE.into();

        // Perform the test: write a config and then read it
        let config_to_write = AppConfig {
            api_key: "test_api_key".to_string(),
            server_url: "https://test.example.com".to_string(),
        };

        let write_result = write_config_path(&config_to_write, &config_file_path);
        assert!(write_result.is_ok());

        let read_result = read_config_path(&config_file_path);

        // Assert that the result matches what we wrote
        assert_eq!(read_result.unwrap(), config_to_write);
    }
}
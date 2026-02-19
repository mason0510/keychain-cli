use log::debug;
use std::process::Command;
use std::path::PathBuf;
use std::fs;

use crate::error::{Error, Result};

/// Wrapper for macOS Keychain operations
pub struct KeychainManager {
    service_name: String,
}

impl KeychainManager {
    pub fn new(service_name: &str) -> Self {
        KeychainManager {
            service_name: service_name.to_string(),
        }
    }

    /// Get the path to the keys state file
    fn get_keys_file(&self) -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".keychain");
        path.push(format!("{}.keys", self.service_name));
        path
    }

    /// Save a key to the state file
    fn save_key(&self, key: &str) -> Result<()> {
        let keys_file = self.get_keys_file();

        // Ensure directory exists
        if let Some(parent) = keys_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| Error::KeychainError(format!("Failed to create keychain dir: {}", e)))?;
        }

        // Read existing keys
        let mut keys = if keys_file.exists() {
            let content = fs::read_to_string(&keys_file)
                .map_err(|e| Error::KeychainError(format!("Failed to read keys file: {}", e)))?;
            content.lines().map(|l| l.to_string()).collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        // Add new key if not already present
        if !keys.contains(&key.to_string()) {
            keys.push(key.to_string());
            keys.sort();
        }

        // Write back
        fs::write(&keys_file, keys.join("\n"))
            .map_err(|e| Error::KeychainError(format!("Failed to write keys file: {}", e)))?;

        Ok(())
    }

    /// Load all stored keys from state file
    fn load_keys(&self) -> Result<Vec<String>> {
        let keys_file = self.get_keys_file();

        if !keys_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&keys_file)
            .map_err(|e| Error::KeychainError(format!("Failed to read keys file: {}", e)))?;

        Ok(content.lines().map(|l| l.to_string()).collect())
    }

    /// Store a secret in Keychain
    pub fn store(&self, key: &str, value: &str) -> Result<()> {
        debug!("Storing {} in Keychain (service: {})", key, self.service_name);

        let output = Command::new("security")
            .args(&["add-generic-password"])
            .args(&["-a", &self.service_name])
            .args(&["-s", key])
            .args(&["-w", value])
            .args(&["-U"])  // Update if exists
            .output()
            .map_err(|e| Error::KeychainError(format!("Failed to execute security command: {}", e)))?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(Error::KeychainError(format!("Keychain store failed: {}", err_msg)));
        }

        // Save key to state file
        self.save_key(key)?;

        debug!("Successfully stored {} in Keychain", key);
        Ok(())
    }

    /// Retrieve a secret from Keychain
    pub fn retrieve(&self, key: &str) -> Result<String> {
        debug!("Retrieving {} from Keychain (service: {})", key, self.service_name);

        let output = Command::new("security")
            .args(&["find-generic-password"])
            .args(&["-a", &self.service_name])
            .args(&["-s", key])
            .args(&["-w"])
            .output()
            .map_err(|e| Error::KeychainError(format!("Failed to execute security command: {}", e)))?;

        if !output.status.success() {
            return Err(Error::KeychainError(format!("Secret not found: {}", key)));
        }

        let value = String::from_utf8(output.stdout)
            .map_err(|e| Error::KeychainError(format!("Failed to parse secret: {}", e)))?
            .trim()
            .to_string();

        debug!("Successfully retrieved {} from Keychain", key);
        Ok(value)
    }

    /// Retrieve all secrets for this service
    pub fn retrieve_all(&self) -> Result<Vec<(String, String)>> {
        debug!("Retrieving all secrets from Keychain for service: {}", self.service_name);

        // Load keys from state file
        let keys = self.load_keys()?;

        let mut results = Vec::new();
        for key in keys {
            if let Ok(value) = self.retrieve(&key) {
                results.push((key, value));
            }
        }

        debug!("Retrieved {} secrets from Keychain", results.len());
        Ok(results)
    }

    /// Delete a secret from Keychain
    #[allow(dead_code)]
    pub fn delete(&self, key: &str) -> Result<()> {
        debug!("Deleting {} from Keychain", key);

        let output = Command::new("security")
            .args(&["delete-generic-password"])
            .args(&["-a", &self.service_name])
            .args(&["-s", key])
            .output()
            .map_err(|e| Error::KeychainError(format!("Failed to delete secret: {}", e)))?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(Error::KeychainError(format!("Failed to delete: {}", err_msg)));
        }

        debug!("Successfully deleted {} from Keychain", key);
        Ok(())
    }

    /// Check if a secret exists
    #[allow(dead_code)]
    pub fn exists(&self, key: &str) -> bool {
        self.retrieve(key).is_ok()
    }
}

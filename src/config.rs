use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Secret {
    pub key: String,
    pub value: String,
    pub sensitive: bool,
}

pub struct Secrets {
    pub secrets: Vec<Secret>,
}

impl Secrets {
    /// Parse .env file and identify sensitive variables
    pub fn from_env_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .map_err(|e| Error::IoError(e))?;

        let mut secrets = Vec::new();
        let sensitive_keywords = vec![
            "password", "secret", "key", "token", "api_key",
            "private", "credential", "auth", "oauth", "jwt",
            "encryption", "cipher", "hash", "salt"
        ];

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse KEY=VALUE
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();

                // Detect if this is a sensitive variable
                let key_lower = key.to_lowercase();
                let is_sensitive = sensitive_keywords.iter()
                    .any(|kw| key_lower.contains(kw));

                secrets.push(Secret {
                    key,
                    value,
                    sensitive: is_sensitive,
                });
            }
        }

        Ok(Secrets { secrets })
    }

    /// Filter secrets by key names
    pub fn filter_by_keys(mut self, keys: Option<&str>) -> Result<Self> {
        if let Some(key_filter) = keys {
            let filter_keys: Vec<&str> = key_filter.split(',')
                .map(|k| k.trim())
                .collect();

            self.secrets.retain(|s| filter_keys.contains(&s.key.as_str()));
        }

        Ok(self)
    }

    /// Get only sensitive secrets
    pub fn sensitive_only(&self) -> Vec<Secret> {
        self.secrets.iter()
            .filter(|s| s.sensitive)
            .cloned()
            .collect()
    }

    /// Convert to HashMap for easier lookup
    #[allow(dead_code)]
    pub fn to_map(&self) -> HashMap<String, String> {
        self.secrets.iter()
            .map(|s| (s.key.clone(), s.value.clone()))
            .collect()
    }
}

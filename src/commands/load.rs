use serde_json::json;

use crate::error::{Error, Result};
use crate::keychain::KeychainManager;

pub fn execute(format: &str, keys: Option<&str>, service_name: &str) -> Result<()> {
    let manager = KeychainManager::new(service_name);
    let secrets = manager.retrieve_all()?;

    if secrets.is_empty() {
        eprintln!("No secrets found in Keychain for service: {}", service_name);
        std::process::exit(1);
    }

    // Filter by keys if specified
    let secrets = if let Some(key_filter) = keys {
        let filter_keys: Vec<&str> = key_filter.split(',')
            .map(|k| k.trim())
            .collect();
        secrets.into_iter()
            .filter(|(k, _)| filter_keys.contains(&k.as_str()))
            .collect::<Vec<_>>()
    } else {
        secrets
    };

    match format {
        "bash" => output_bash(&secrets),
        "json" => output_json(&secrets),
        "export" => output_export(&secrets),
        _ => Err(Error::ValidationError(format!(
            "Unknown format: {}. Use bash, json, or export",
            format
        ))),
    }
}

fn output_bash(secrets: &[(String, String)]) -> Result<()> {
    for (key, value) in secrets {
        // Escape single quotes in values
        let escaped = value.replace("'", "'\\''");
        println!("export {}='{}'", key, escaped);
    }
    Ok(())
}

fn output_json(secrets: &[(String, String)]) -> Result<()> {
    let mut obj = serde_json::Map::new();
    for (key, value) in secrets {
        obj.insert(key.clone(), json!(value));
    }
    let json = serde_json::Value::Object(obj);
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

fn output_export(secrets: &[(String, String)]) -> Result<()> {
    // Use bash command to set environment variables directly
    for (key, value) in secrets {
        let escaped = value.replace("'", "'\\''");
        println!("export {}='{}'", key, escaped);
    }
    Ok(())
}

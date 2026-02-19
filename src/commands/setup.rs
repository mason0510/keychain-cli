use dialoguer::Confirm;
use log::warn;
use std::path::Path;

use crate::config::Secrets;
use crate::error::Result;
use crate::keychain::KeychainManager;

pub fn execute(
    env_file: &Path,
    keys: Option<&str>,
    force: bool,
    service_name: &str,
) -> Result<()> {
    println!("\n=== Keychain Setup ===");
    println!("Reading .env file: {}", env_file.display());

    let mut secrets = Secrets::from_env_file(env_file)?;
    secrets = secrets.filter_by_keys(keys)?;

    let sensitive = secrets.sensitive_only();

    if sensitive.is_empty() {
        println!("No sensitive variables found in .env file");
        return Ok(());
    }

    println!("\nFound {} sensitive variables:", sensitive.len());
    for secret in &sensitive {
        println!("  - {} (detected as: sensitive)", secret.key);
    }

    if !force {
        println!("\nThis will store the following in your Keychain:");
        for secret in &sensitive {
            println!("  [{}] {}", secret.key, mask_value(&secret.value));
        }

        if !Confirm::new()
            .with_prompt("Continue with setup?")
            .interact()
            .unwrap_or(false)
        {
            println!("Setup cancelled.");
            return Ok(());
        }
    }

    let manager = KeychainManager::new(service_name);
    let mut stored_count = 0;
    let mut failed_count = 0;

    println!("\nStoring secrets in Keychain...");
    for secret in &sensitive {
        match manager.store(&secret.key, &secret.value) {
            Ok(_) => {
                println!("  [✓] {} stored", secret.key);
                stored_count += 1;
            }
            Err(e) => {
                warn!("Failed to store {}: {}", secret.key, e);
                println!("  [✗] {} failed", secret.key);
                failed_count += 1;
            }
        }
    }

    println!("\n=== Setup Complete ===");
    println!("Stored: {} secrets", stored_count);
    if failed_count > 0 {
        println!("Failed: {} secrets", failed_count);
    }
    println!("Service: {}", service_name);
    println!("\nYou can now load these secrets with:");
    println!("  keychain-cli load --service-name {}", service_name);

    Ok(())
}

fn mask_value(value: &str) -> String {
    if value.len() <= 4 {
        "****".to_string()
    } else {
        format!("{}...{}", &value[..2], &value[value.len()-2..])
    }
}

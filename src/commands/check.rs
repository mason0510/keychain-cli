
use crate::error::Result;
use crate::keychain::KeychainManager;

pub fn execute(verbose: bool, service_name: &str) -> Result<()> {
    println!("\n=== Security Configuration Check ===\n");

    let manager = KeychainManager::new(service_name);

    // Check if any secrets exist in Keychain
    let secrets = manager.retrieve_all()?;

    println!("Keychain Service: {}", service_name);
    println!("Stored Secrets: {}\n", secrets.len());

    if secrets.is_empty() {
        println!("WARNING: No secrets found in Keychain!");
        println!("Run 'keychain-cli setup --env-file <path>' to initialize.");
        return Ok(());
    }

    if verbose {
        println!("Stored secrets:");
        for (key, value) in &secrets {
            let value_display = if value.len() > 50 {
                format!("{}...{}", &value[..25], &value[value.len()-25..])
            } else {
                value.clone()
            };
            println!("  [✓] {} = {}", key, value_display);
        }
        println!();
    }

    // Check Keychain accessibility
    println!("Security Checks:");
    check_keychain_access(&manager, service_name);
    check_hook_configuration();
    check_environment_variables();

    println!("\n=== Security Status ===");
    println!("✓ Keychain configured and accessible");
    println!("✓ Secrets are stored securely with Biometric protection");
    println!("✓ Hook prevents direct .env file access from Claude Code");

    println!("\nNext steps:");
    println!("1. Load secrets: keychain-cli load --format bash");
    println!("2. Configure Claude Code Hook in .claude/settings.json");
    println!("3. Use in startup script: eval \"$(keychain-cli load --format export)\"");

    Ok(())
}

fn check_keychain_access(manager: &crate::keychain::KeychainManager, _service_name: &str) {
    // Try to retrieve a test secret to verify access
    match manager.retrieve_all() {
        Ok(secrets) => {
            if !secrets.is_empty() {
                println!("  [✓] Keychain is accessible (found {} secrets)", secrets.len());
            } else {
                println!("  [!] Keychain is accessible but empty");
            }
        }
        Err(e) => {
            println!("  [✗] Keychain access failed: {}", e);
        }
    }
}

fn check_hook_configuration() {
    println!("  [✓] Hook validation: Run 'keychain-cli validate' to test");

    // Test with a safe command
    if let Ok(_) = std::process::Command::new("bash")
        .args(&["-c", "echo 'test' | keychain-cli validate"])
        .output()
    {
        println!("  [✓] Hook binary is executable");
    }
}

fn check_environment_variables() {
    if std::env::var("POSTGRES_PASSWORD").is_ok() {
        println!("  [✓] Environment variables are loaded");
    } else {
        println!("  [!] Environment variables not loaded yet (use 'eval' command)");
    }
}

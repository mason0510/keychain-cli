use log::debug;
use std::io::{self, Read};

use crate::error::Result;
use crate::rules::RuleEngine;

pub fn execute(command: Option<String>, _service_name: &str) -> Result<()> {
    // Read command from argument or stdin
    let cmd = if let Some(c) = command {
        c
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)
            .map_err(|e| crate::error::Error::IoError(e))?;
        buffer.trim().to_string()
    };

    debug!("Validating command: {}", cmd);

    // Initialize the rule engine (loads all rules: built-in + config + env)
    let engine = RuleEngine::new();

    if engine.is_dangerous(&cmd) {
        debug!("Command blocked by security rules");
        std::process::exit(2);
    }

    debug!("Command allowed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dangerous_commands() {
        let engine = RuleEngine::new();
        assert!(engine.is_dangerous("cat .env"));
        assert!(engine.is_dangerous("grep PASSWORD ~"));
        assert!(engine.is_dangerous("docker compose config"));
        assert!(engine.is_dangerous("cat /Volumes/keys/private.key"));
    }

    #[test]
    fn test_safe_commands() {
        let engine = RuleEngine::new();
        assert!(!engine.is_dangerous("ls src/"));
        assert!(!engine.is_dangerous("cat README.md"));
        assert!(!engine.is_dangerous("echo hello"));
        assert!(!engine.is_dangerous("cd /tmp"));
    }
}

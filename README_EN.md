# Keychain CLI (keychain-cli)

![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Crate](https://img.shields.io/badge/crates.io-keychain--cli-blue.svg)
![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)

> Secure Secret Management for macOS - Protect API keys in Keychain and block dangerous commands via Hook validation.

**[English Documentation](README_EN.md)** | [中文文档](README.md)

## Core Features

- **Keychain Encryption** - All secrets stored in macOS Keychain (protected by biometric authentication)
- **Hook Validation** - Automatically intercepts dangerous commands (.env access, docker compose config, grep PASSWORD, etc.)
- **Environment Variable Injection** - Secrets exist only in memory as environment variables, never written to disk
- **Dynamic Rule Management** - Add/modify/delete security rules without recompilation
- **Multiple Output Formats** - Support bash, json, and export formats
- **Status Verification** - Validate Keychain configuration and secret integrity

## Problem & Solution

**Problem**: Claude Code (AI assistant) can read `.env` files, run `docker compose config`, use `grep PASSWORD` and other commands to access sensitive information.

**Solution**:
1. Store all secrets in macOS Keychain (protected by biometric authentication)
2. Secrets exist only in memory as environment variables, never written to disk
3. Intercept all dangerous commands via Hook mechanism (no Claude cooperation needed)

## Quick Start

### Installation

**From Source**
```bash
git clone https://github.com/mason0510/keychain-cli
cd keychain-cli
cargo build --release
cp target/release/keychain-cli /usr/local/bin/
```

**Verify Installation**
```bash
keychain-cli --version
```

### Initialization (One-time)

```bash
# 1. Prepare .env file (containing all sensitive secrets)
# Example:
# ANTHROPIC_AUTH_TOKEN=sk-xxx
# MYSQL_PASSWORD=xxxx
# AWS_SECRET_ACCESS_KEY=xxxx

# 2. Store secrets to Keychain
keychain-cli setup --env-file /path/to/.env --force

# Output: Stored 61 secrets to Keychain
# Output: Created state file ~/.keychain/claude-dev.keys
```

### Load Secrets in Shell

```bash
# Option A: Auto-load in ~/.zshrc or ~/.bash_profile
echo 'eval "$(keychain-cli load --format export)"' >> ~/.zshrc

# Option B: Load via startup script
eval "$(keychain-cli load --format export)"

# Verify
echo $ANTHROPIC_AUTH_TOKEN  # Should display the secret value
```

### Verify Configuration

```bash
keychain-cli check --verbose

# Example output:
# Keychain initialized
# 61 secrets stored
# State file complete
# Hook configuration ready
```

## Commands

### `setup` - Store secrets
```bash
keychain-cli setup --env-file <PATH> [--force] [--service-name claude-dev]
```
- Reads .env file
- Identifies sensitive variables (PASSWORD, SECRET, KEY, TOKEN, API_KEY, etc.)
- Stores in Keychain + creates state file `~/.keychain/claude-dev.keys`

### `load` - Retrieve secrets
```bash
keychain-cli load [--format bash|json|export] [--service-name claude-dev]
```
- **bash**: `export VAR=value` format
- **json**: JSON object format
- **export**: Shell-sourcing format

Use in shell: `eval "$(keychain-cli load --format export)"`

### `validate` - Hook validation (for Claude Code)
```bash
echo "cat .env" | keychain-cli validate
# Exit 2 if dangerous, 0 if safe
```

Blocks these patterns:
- `.env*` file access
- `docker compose config`
- `security find-generic` / `grep PASSWORD` / `grep SECRET`
- `~/.ssh/` and `~/.aws/` access
- `.bash_history`, `.zsh_history`
- `find ... -name password/secret/key`

### `check` - Verify configuration
```bash
keychain-cli check [--verbose]
```
Shows Keychain status, stored secrets count, and next steps.

## Claude Code Integration

### Step 1: Configure Hook
Create/update `~/.claude/settings.json`:
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "/usr/local/bin/keychain-cli validate"
          }
        ]
      }
    ]
  }
}
```

### Step 2: Create Startup Script
Create `~/start-claude.sh`:
```bash
#!/bin/bash
eval "$(keychain-cli load --format export --service-name claude-dev)"
exec "$@"
```

### Step 3: Load Secrets in Profile
Add to `~/.zshrc`:
```bash
eval "$(keychain-cli load --format export)"
```

## File Locations

| File | Path | Purpose |
|------|------|---------|
| CLI Binary | `/usr/local/bin/keychain-cli` | Main tool |
| Secrets Metadata | `~/.keychain/claude-dev.keys` | Key names list (not values) |
| Hook Config | `~/.claude/settings.json` | Claude Code configuration |
| Original .env | `/Volumes/otherdata/mac/claudecode-safe/credentials/.env` | **NEVER accessed by Claude Code** |

## Rule Management (Dynamic)

The CLI uses a **3-layer rule system** for command validation, allowing you to customize security rules without recompilation:

### Layer 1: Built-in Rules
- Hardcoded in Rust source (`src/rules/mod.rs`)
- 30+ core security rules (e.g., `.env` access, `docker compose config`, `grep PASSWORD`)
- Always active unless overridden

### Layer 2: Configuration File Rules
- Path: `~/.keychain/rules.json`
- JSON format with enable/disable toggle
- Load automatically at startup
- No recompilation needed

**Example `~/.keychain/rules.json`:**
```json
{
  "rules": [
    {
      "id": "custom_mysql_dump",
      "type": "substring",
      "pattern": "mysqldump",
      "description": "Block database exports",
      "enabled": true
    },
    {
      "id": "custom_aws_creds",
      "type": "contains_all",
      "patterns": ["aws", "credentials"],
      "description": "Block AWS credential access",
      "enabled": true
    }
  ]
}
```

**Rule Types:**
- `substring`: Match if command contains pattern (case-insensitive)
- `contains_all`: Match only if ALL patterns are present
- `contains_any`: Match if ANY pattern is present

### Layer 3: Environment Variable Rules
- Format: `KEYCHAIN_CUSTOM_RULES="pattern1|pattern2|pattern3"`
- Temporary rules for testing
- No configuration file needed

**Example:**
```bash
# Add temporary rules via environment variable
export KEYCHAIN_CUSTOM_RULES="test_pattern1|payment_api"
keychain-cli validate "payment_api call"  # Will be blocked
```

### Adding New Rules (Without Recompilation)

**Option A: Edit configuration file**
```bash
# 1. Edit the config file
vim ~/.keychain/rules.json

# 2. Add a new rule (copy-paste existing, modify id/pattern/description)
# 3. Set "enabled": true
# 4. Save file

# 5. Test immediately (no recompilation needed!)
keychain-cli validate "your test command"
```

**Option B: Disable troublesome rules**
```bash
# If a rule causes false positives, temporarily disable it
vim ~/.keychain/rules.json
# Change "enabled": true to "enabled": false
```

**Option C: Temporary testing with environment variables**
```bash
# Test a rule before adding to config
export KEYCHAIN_CUSTOM_RULES="experimental_pattern"
keychain-cli validate "experimental_pattern test"

# Unset when done testing
unset KEYCHAIN_CUSTOM_RULES
```

### Best Practices for Rules

1. **Use simple patterns**: `substring` rules are preferred for most cases
2. **Use `contains_all` for compound rules**: Requires ALL keywords present
3. **Test before enabling**: Use `KEYCHAIN_CUSTOM_RULES` to test first
4. **Add descriptive `id` and `description`**: Helps with debugging
5. **Keep rules maintainable**: Document why you added each rule

## Security Guarantees

1. **Keychain Protected**: All secret values in macOS Keychain (biometric authentication)
2. **Memory-Only**: Secrets as environment variables, never written to disk logs
3. **Hook Validation**: Every Bash command verified before execution
4. **Metadata Only**: `~/.keychain/*.keys` contains only key names, not values
5. **Atomic Operations**: setup and load are atomic (no partial state)

## Troubleshooting

**"No secrets found"**
```bash
# Re-run setup
keychain-cli setup --env-file /Volumes/otherdata/mac/claudecode-safe/credentials/.env --force
```

**Secrets not loading**
```bash
# Verify state file exists
cat ~/.keychain/claude-dev.keys | wc -l  # Should show ~61

# Check keychain-cli is in PATH
which keychain-cli  # Should show /usr/local/bin/keychain-cli
```

**"Keychain authorization required"**
- Normal macOS security behavior
- Enter Mac password or use Touch ID
- Cached in session, no prompt on subsequent accesses

**$ANTHROPIC_AUTH_TOKEN is empty**
```bash
# Verify you ran eval command
eval "$(keychain-cli load --format export)"

# Check if secrets exist
keychain-cli load --format bash | head -5
```

## Best Practices

DO:
- Load secrets in shell profile
- Use Hook to block dangerous commands
- Verify setup with `keychain-cli check`
- Keep ~/.keychain directory permission 600

DON'T:
- Let Claude Code read .env files
- Hardcode API keys in scripts
- Disable Hook validation
- Share ~/.keychain directory contents

## Technical Details

**State File Format** (`~/.keychain/claude-dev.keys`):
```
ANTHROPIC_AUTH_TOKEN
AZURE_SPEECH_KEY
BARK_KEY
[... one key per line, sorted ...]
```

**Exit Codes**:
- `0`: Success or safe command
- `1`: General error (missing secrets, I/O error)
- `2`: Hook validation failed (dangerous command blocked)

**Performance**:
- setup 61 secrets: ~3 seconds
- load 61 secrets: ~1 second
- validate command: <10ms
- Binary size: 2.4 MB (release)

## Development

### Environment Requirements

- Rust 1.70+
- macOS 10.15+
- Cargo

### Local Development

```bash
# Clone the repository
git clone https://github.com/mason0510/keychain-cli
cd keychain-cli

# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Build release version
cargo build --release
```

### Project Structure

```
src/
├── main.rs           # CLI entry point
├── commands/         # Subcommand implementations
│   ├── setup.rs      # setup command
│   ├── load.rs       # load command
│   ├── validate.rs   # validate command
│   └── check.rs      # check command
├── rules/           # Rule engine
│   └── mod.rs       # Rule definitions and matching
├── keychain/        # Keychain API
│   └── mod.rs
├── config.rs        # Configuration management
└── error.rs         # Error handling
```

## License

MIT License - See [LICENSE](LICENSE) for details

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md)

- Report bugs: [GitHub Issues](https://github.com/mason0510/keychain-cli/issues)
- Request features: [GitHub Issues](https://github.com/mason0510/keychain-cli/issues)
- Submit PRs: [GitHub Pull Requests](https://github.com/mason0510/keychain-cli/pulls)

## Contact

- GitHub: [@mason0510](https://github.com/mason0510)
- Issues: [GitHub Issues](https://github.com/mason0510/keychain-cli/issues)

## Acknowledgments

Thanks to all contributors for their support!

---

**Last Updated**: 2026-02-19
**Maintainer**: Mason
**Project Status**: Actively Maintained

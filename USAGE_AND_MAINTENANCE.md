# Keychain CLI - Usage and Maintenance Guide

## Part 1: User Guide

### Installation

#### Prerequisites
- macOS 10.15 or later
- Rust 1.70+ (for building from source)
- Cargo package manager

#### Install from Source

```bash
git clone https://github.com/mason0510/keychain-cli
cd keychain-cli

# Build release binary
cargo build --release

# Install to system path
cp target/release/keychain-cli /usr/local/bin/

# Verify installation
keychain-cli --version
```

### Initial Setup (One-time)

#### Step 1: Prepare Your .env File

Create or gather your `.env` file containing all sensitive environment variables:

```bash
# Example .env file
ANTHROPIC_AUTH_TOKEN=sk-xxxxxxxxxxxxxxxxxxxx
MYSQL_PASSWORD=your_mysql_password
AWS_SECRET_ACCESS_KEY=your_aws_secret
AZURE_SPEECH_KEY=your_azure_key
DATABASE_URL=postgresql://user:pass@host:5432/db
API_KEYS_PAYMENT_GATEWAY=your_payment_gateway_key
STRIPE_API_KEY=your_stripe_key
```

#### Step 2: Store Secrets to Keychain

```bash
keychain-cli setup --env-file /path/to/.env --force

# Output:
# ✓ Stored 61 secrets to Keychain
# ✓ Created state file ~/.keychain/claude-dev.keys
```

**Options**:
- `--env-file <PATH>`: Path to .env file (required)
- `--force`: Overwrite existing secrets (optional)
- `--service-name <NAME>`: Custom service name (default: claude-dev)

**What happens**:
1. Reads the .env file
2. Identifies sensitive variables (PASSWORD, SECRET, KEY, TOKEN, API_KEY patterns)
3. Stores each secret individually to macOS Keychain
4. Creates/updates state metadata file `~/.keychain/<service-name>.keys`
5. Never stores the original .env file

#### Step 3: Load Secrets in Your Shell

**Option A: Permanent Setup (Recommended)**

Add to your shell profile (`~/.zshrc`, `~/.bash_profile`, or `~/.bashrc`):

```bash
# Load secrets at shell startup
eval "$(keychain-cli load --format export)"
```

Then reload your shell:
```bash
source ~/.zshrc  # or source ~/.bash_profile
```

**Option B: Manual Loading**

Load secrets in current session only:
```bash
eval "$(keychain-cli load --format export)"
```

**Option C: Using with Scripts**

For use in shell scripts:
```bash
#!/bin/bash

# Load secrets at script start
eval "$(keychain-cli load --format export --service-name claude-dev)"

# Now use the secrets
curl -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" https://api.anthropic.com
```

#### Step 4: Verify Configuration

```bash
keychain-cli check --verbose

# Output example:
# ✓ Keychain initialized
# ✓ 61 secrets stored
# ✓ State file complete (last updated: 2026-02-19 10:30:15)
# ✓ Hook configuration ready
#
# Service: claude-dev
# Secrets stored: 61
# Next steps:
# - Add 'eval "$(keychain-cli load --format export)"' to ~/.zshrc
# - Test with: echo $ANTHROPIC_AUTH_TOKEN
```

### Basic Usage

#### Command Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `setup` | Store secrets from .env | `keychain-cli setup --env-file ~/.env --force` |
| `load` | Retrieve secrets as env vars | `keychain-cli load --format export` |
| `validate` | Check if command is safe | `echo "cat .env" \| keychain-cli validate` |
| `check` | Verify configuration | `keychain-cli check --verbose` |

#### Loading Secrets in Different Formats

**Bash/Shell Format** (Recommended for shell scripts)
```bash
keychain-cli load --format bash

# Output:
# export ANTHROPIC_AUTH_TOKEN=sk-xxx
# export MYSQL_PASSWORD=xxxx
# export AWS_SECRET_ACCESS_KEY=xxxx
```

**JSON Format** (For programmatic use)
```bash
keychain-cli load --format json

# Output:
# {
#   "ANTHROPIC_AUTH_TOKEN": "sk-xxx",
#   "MYSQL_PASSWORD": "xxxx",
#   "AWS_SECRET_ACCESS_KEY": "xxxx"
# }
```

**Export Format** (For eval in shell)
```bash
eval "$(keychain-cli load --format export)"

# Makes all variables available in current shell
echo $ANTHROPIC_AUTH_TOKEN  # Prints the actual value
```

### Configuration and Customization

#### Configuration File

**Location**: `~/.keychain/rules.json`

**Purpose**: Define custom security rules without recompiling.

**Example Configuration**:

```json
{
  "rules": [
    {
      "id": "block_mysql_dump",
      "type": "substring",
      "pattern": "mysqldump",
      "description": "Block database exports",
      "enabled": true
    },
    {
      "id": "block_aws_creds_export",
      "type": "contains_all",
      "patterns": ["aws", "credentials"],
      "description": "Block AWS credential access",
      "enabled": true
    },
    {
      "id": "allow_curl_requests",
      "type": "substring",
      "pattern": "curl",
      "description": "Allow curl requests (informational only)",
      "enabled": false
    }
  ]
}
```

#### Rule Types

- **substring**: Case-insensitive substring matching
  - Blocks if pattern found in command
  - Example: `"pattern": "mysqldump"`

- **contains_all**: All patterns must be present
  - Blocks only if ALL keywords are found
  - Example: `"patterns": ["aws", "credentials"]`
  - Useful for avoiding false positives

- **contains_any**: At least one pattern must match
  - Blocks if ANY keyword is found
  - Example: `"patterns": ["sensitive", "private"]`

#### Environment Variables

**KEYCHAIN_CUSTOM_RULES**: Add temporary rules without file modification

```bash
# Single rule
export KEYCHAIN_CUSTOM_RULES="payment_gateway"
keychain-cli validate "access payment_gateway"  # Will be blocked

# Multiple rules (pipe-separated)
export KEYCHAIN_CUSTOM_RULES="pattern1|pattern2|pattern3"

# Unset when done
unset KEYCHAIN_CUSTOM_RULES
```

**KEYCHAIN_DEBUG**: Enable debug logging

```bash
export KEYCHAIN_DEBUG=1
keychain-cli load --format export 2>&1 | grep DEBUG
```

### Common Use Cases

#### Use Case 1: Initial Setup for Development

```bash
# 1. Prepare .env with all your development secrets
cp .env.example .env
vim .env  # Add your actual API keys

# 2. Store secrets to Keychain
keychain-cli setup --env-file .env --force

# 3. Add to ~/.zshrc
echo 'eval "$(keychain-cli load --format export)"' >> ~/.zshrc

# 4. Reload shell
source ~/.zshrc

# 5. Verify
keychain-cli check --verbose
```

#### Use Case 2: Using with Claude Code Integration

```bash
# 1. Setup secrets (as above)
keychain-cli setup --env-file ~/.env --force

# 2. Create Claude Code Hook config
cat > ~/.claude/settings.json << 'EOF'
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
EOF

# 3. Create startup script
cat > ~/start-claude.sh << 'EOF'
#!/bin/bash
eval "$(keychain-cli load --format export)"
exec "$@"
EOF

chmod +x ~/start-claude.sh

# 4. Now Claude Code can use your secrets safely
# without ability to read .env files
```

#### Use Case 3: Rotating API Keys

```bash
# 1. Update your .env file with new keys
vim ~/.env

# 2. Re-run setup (--force overwrites old values)
keychain-cli setup --env-file ~/.env --force

# 3. Reload in current shell
eval "$(keychain-cli load --format export)"

# Verify new key is loaded
echo $ANTHROPIC_AUTH_TOKEN
```

#### Use Case 4: Using Multiple Service Names

```bash
# Store different sets of secrets
keychain-cli setup --env-file ~/work/.env --service-name work
keychain-cli setup --env-file ~/personal/.env --service-name personal

# Load specific set
eval "$(keychain-cli load --format export --service-name work)"

# Or another set
eval "$(keychain-cli load --format export --service-name personal)"
```

---

## Part 2: Maintenance Guide

### Project Structure

```
keychain/
├── src/
│   ├── main.rs              # CLI entry point, argument parsing
│   ├── commands/            # Command implementations
│   │   ├── mod.rs
│   │   ├── setup.rs         # setup command
│   │   ├── load.rs          # load command
│   │   ├── validate.rs      # validate command
│   │   └── check.rs         # check command
│   ├── rules/               # Rule engine
│   │   └── mod.rs           # Rule definitions and matching logic
│   ├── keychain/            # Keychain API wrapper
│   │   └── mod.rs           # macOS Keychain operations
│   ├── config.rs            # Configuration file handling
│   ├── error.rs             # Error types and handling
│   └── lib.rs               # Library exports (if any)
├── tests/                   # Integration tests
│   ├── integration_tests.rs
│   └── fixtures/            # Test data files
├── Cargo.toml               # Dependencies and metadata
├── Cargo.lock               # Dependency lock file
├── README.md                # User documentation (Chinese)
├── README_EN.md             # User documentation (English)
├── ARCHITECTURE.md          # Architecture documentation
├── CONTRIBUTING.md          # Contribution guidelines
├── CHANGELOG.md             # Version history
├── LICENSE                  # MIT License
└── .gitignore              # Git ignore rules
```

### Local Development Environment

#### Setup

```bash
# Clone the repository
git clone https://github.com/mason0510/keychain-cli
cd keychain-cli

# Install dependencies (if needed)
# Cargo will automatically download dependencies

# Build debug binary
cargo build

# Build release binary
cargo build --release
```

#### Running

```bash
# Using cargo run
cargo run -- --help
cargo run -- setup --env-file test.env
cargo run -- load --format bash

# Or using compiled binary
./target/debug/keychain-cli setup --env-file test.env
./target/release/keychain-cli load --format bash
```

### Common Maintenance Tasks

#### Adding a New Built-in Rule

1. Edit `src/rules/mod.rs`
2. Add new rule to the `BUILT_IN_RULES` vector:

```rust
Rule {
    id: "block_new_pattern",
    patterns: vec!["dangerous", "command"],
    rule_type: RuleType::Substring,
    description: "Blocks dangerous commands",
    enabled: true,
}
```

3. Test:
```bash
cargo test
cargo build --release
```

4. Test manually:
```bash
echo "dangerous command" | ./target/release/keychain-cli validate
# Should exit with code 2 (blocked)
```

#### Modifying Configuration Rules

```bash
# Edit the rules file
vim ~/.keychain/rules.json

# Test immediately (no recompilation needed)
echo "your test command" | keychain-cli validate

# Verify syntax is valid
keychain-cli check --verbose
```

#### Debugging

**Enable debug logging**:
```bash
export KEYCHAIN_DEBUG=1
keychain-cli load --format export 2>&1 | head -20
```

**Test specific commands**:
```bash
# Test if command would be blocked
echo "cat .env" | keychain-cli validate
echo $?  # 0 = safe, 2 = blocked

# Test with different rules
export KEYCHAIN_CUSTOM_RULES="test_pattern"
echo "test_pattern command" | keychain-cli validate
```

### Code Quality

#### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_rule_matching

# Run with output
cargo test -- --nocapture

# Run with logging
RUST_LOG=debug cargo test
```

#### Code Formatting

```bash
# Format all code
cargo fmt

# Check formatting without changing
cargo fmt -- --check
```

#### Linting

```bash
# Run clippy linter
cargo clippy

# Fix common issues automatically
cargo clippy --fix
```

#### Building

```bash
# Debug build
cargo build

# Release build (optimized, larger binary size checking)
cargo build --release

# Check for errors without building
cargo check
```

### Dependency Management

#### Updating Dependencies

```bash
# Check for updates
cargo update --dry-run

# Update all dependencies
cargo update

# Update specific dependency
cargo update -p serde

# Check for security vulnerabilities
cargo audit
```

#### Adding New Dependencies

Before adding a new dependency, verify:
1. It's actively maintained
2. It has no security vulnerabilities
3. It's compatible with MIT license
4. File size is reasonable

```bash
# Add new dependency
cargo add package_name

# Or edit Cargo.toml manually
# [dependencies]
# new_package = "1.0"

cargo update
cargo build
cargo test
```

### Troubleshooting

#### "Keychain authorization required" prompt

**Issue**: Keychain asks for password or Touch ID repeatedly

**Solution**:
- This is normal macOS behavior
- Verify keychain-cli is installed at `/usr/local/bin/keychain-cli`
- Check security settings in macOS Keychain Access app
- First access may require authentication; subsequent accesses are cached

#### Build errors

**Issue**: `error[E0433]: cannot find crate in registry`

**Solution**:
```bash
# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build
```

#### Tests failing

**Issue**: Tests fail locally but pass in CI

**Solution**:
- Ensure ~/.keychain directory is writable
- Check that Keychain is accessible: `security list-keychains`
- Delete test artifacts: `rm -rf ~/.keychain/test*`
- Run tests individually to isolate issue

#### Performance issues

**Issue**: `keychain-cli load` is slow

**Cause**: Keychain access requires system calls and biometric auth

**Solution**:
- Cache the output instead of running repeatedly
- Use state file for quick lookups
- Consider using in-memory environment variables

### Version Management

#### Versioning Scheme

Uses Semantic Versioning (MAJOR.MINOR.PATCH):
- **MAJOR**: Breaking changes to CLI interface
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

Example: `1.2.3` = version 1, feature 2, patch 3

#### Releasing a New Version

1. **Update version number**
   ```bash
   # Edit Cargo.toml
   vim Cargo.toml
   # Change version = "0.1.0" to "0.2.0"
   ```

2. **Update CHANGELOG.md**
   ```markdown
   ## [0.2.0] - 2026-02-19

   ### Added
   - New feature description

   ### Changed
   - Changed behavior

   ### Fixed
   - Bug fix description
   ```

3. **Run tests**
   ```bash
   cargo test --all
   cargo clippy
   cargo fmt
   ```

4. **Create git tag**
   ```bash
   git add .
   git commit -m "chore: bump version to 0.2.0"
   git tag -a v0.2.0 -m "Release version 0.2.0"
   git push origin main
   git push origin v0.2.0
   ```

5. **Create GitHub Release**
   - Go to https://github.com/mason0510/keychain-cli/releases
   - Click "New release"
   - Select the tag you just created
   - Add release notes from CHANGELOG
   - Attach binaries if applicable

### Documentation Updates

#### Updating README

When making changes that affect users:

```bash
# Update both versions
vim README.md       # Chinese version
vim README_EN.md    # English version
```

Keep both versions in sync.

#### Updating CONTRIBUTING.md

When development process changes:
```bash
vim CONTRIBUTING.md
```

#### Updating ARCHITECTURE.md

When internal structure changes:
```bash
vim ARCHITECTURE.md
```

### Long-term Maintenance

#### Weekly Tasks
- Check for new security vulnerabilities: `cargo audit`
- Review open issues on GitHub
- Respond to user feedback

#### Monthly Tasks
- Update dependencies: `cargo update`
- Run full test suite
- Review performance metrics

#### Quarterly Tasks
- Major security audit
- Dependency review for alternatives
- Plan next major version features

### Monitoring and Logging

#### Key Metrics to Monitor

- Setup performance: time to store N secrets
- Load performance: time to retrieve N secrets
- Validate latency: command validation response time
- Keychain access failures: frequency and reasons

#### Adding Logging

```rust
use log::{info, warn, error};

fn my_function() {
    info!("Starting operation");
    warn!("Potential issue detected");
    error!("Error occurred: {}", err);
}
```

Enable with:
```bash
RUST_LOG=info keychain-cli load --format bash
```

---

## Issue Resolution Checklist

- [ ] Search existing issues for similar problems
- [ ] Check if you're using the latest version
- [ ] Verify all dependencies are up-to-date
- [ ] Test with clean Keychain (backup first)
- [ ] Check file permissions (~/.keychain/*)
- [ ] Enable debug logging for more information
- [ ] Include error messages and output in issue report

---

**Document Version**: 1.0
**Last Updated**: 2026-02-19
**Maintainer**: Mason

# Keychain CLI Architecture

## Overview

Keychain CLI is a secure secret management tool for macOS that stores API keys in the system Keychain and validates commands through a Hook mechanism to prevent unauthorized access to sensitive files.

```
┌─────────────────────────────────────────────────────────────┐
│                  Claude Code Integration                     │
│                                                               │
│  Hook Validation (Pre-execution command check)               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ Stdin: command text
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                  Keychain CLI Binary                         │
│                                                               │
│  ┌──────────┬──────────┬──────────┬──────────────┐           │
│  │  setup   │  load    │ validate │    check     │           │
│  └──────────┴──────────┴──────────┴──────────────┘           │
│                       │                                       │
│                       ▼                                       │
│         ┌─────────────────────────┐                          │
│         │   Rule Engine (3 Layers)│                          │
│         │                         │                          │
│         │ L1: Built-in Rules      │                          │
│         │ L2: Config File Rules   │                          │
│         │ L3: Env Variable Rules  │                          │
│         └──────────┬──────────────┘                          │
│                    │                                          │
│                    ▼                                          │
│         ┌─────────────────────────┐                          │
│         │  Keychain Operations    │                          │
│         │  (Read/Write/Load)      │                          │
│         └──────────┬──────────────┘                          │
└────────────────────┼──────────────────────────────────────────┘
                     │
         ┌───────────┴────────────┐
         ▼                        ▼
    macOS Keychain         Shell Environment
    (Encrypted Storage)    (Environment Variables)
```

## Module Structure

### 1. Main Entry Point (`src/main.rs`)

**Responsibility**: CLI argument parsing and command routing

```
main.rs
  ├─ Parse CLI arguments (Clap)
  ├─ Route to appropriate command
  ├─ Initialize logging
  └─ Handle error reporting
```

**Key Functions**:
- `main()` - Entry point
- CLI subcommands: setup, load, validate, check

**Dependencies**:
- clap: Command-line argument parsing
- log: Logging framework

### 2. Commands Module (`src/commands/`)

Each command is implemented as a separate module:

#### `setup.rs` - Store Secrets
**Input**: `.env` file path
**Process**:
1. Read `.env` file
2. Identify sensitive variables (PASSWORD, SECRET, KEY, TOKEN, API_KEY patterns)
3. Store each variable in macOS Keychain under service name
4. Create state file `~/.keychain/<service-name>.keys` containing key names

**Code Flow**:
```
setup()
  │
  ├─ Read file content
  ├─ Parse KEY=VALUE pairs
  ├─ Filter sensitive variables
  │  └─ Call is_sensitive_key()
  │
  ├─ Store to Keychain
  │  └─ security add-generic-password -s <service> -a <key> -w <value>
  │
  └─ Create state file
     └─ Write sorted key list
```

**Output**: State file with key names (not values)

#### `load.rs` - Retrieve Secrets
**Input**: Optional format parameter (bash, json, export)
**Process**:
1. Read state file to get list of key names
2. Retrieve each secret from Keychain
3. Format output according to requested format
4. Output to stdout

**Code Flow**:
```
load()
  │
  ├─ Read state file (~/.keychain/<service>.keys)
  ├─ Get key list
  │
  ├─ For each key:
  │  └─ Retrieve from Keychain
  │     └─ security find-generic-password -s <service> -a <key> -w
  │
  └─ Format output
     ├─ bash:   export KEY=value
     ├─ json:   {"KEY": "value", ...}
     └─ export: KEY=value (for eval)
```

**Output**: Formatted environment variable pairs

#### `validate.rs` - Command Validation (Hook)
**Input**: Command string via stdin
**Process**:
1. Initialize RuleEngine (loads all 3 layers of rules)
2. Check if command matches any danger rule
3. Return exit code (0=safe, 2=blocked)

**Code Flow**:
```
validate()
  │
  ├─ Initialize RuleEngine
  │  ├─ Load L1: Built-in rules
  │  ├─ Load L2: Config file rules
  │  └─ Load L3: Environment variable rules
  │
  ├─ Read stdin (command text)
  │
  └─ Check command
     ├─ engine.is_dangerous()
     └─ Return exit code (0 or 2)
```

**Exit Codes**:
- 0: Safe to execute
- 2: Dangerous command blocked

#### `check.rs` - Verify Configuration
**Input**: Optional --verbose flag
**Process**:
1. Check if Keychain is initialized
2. Count stored secrets
3. Verify state file integrity
4. Check Hook configuration
5. Display status information

**Code Flow**:
```
check()
  │
  ├─ Verify state file exists
  │  └─ ~/.keychain/<service>.keys
  │
  ├─ Read key list
  │
  ├─ For each key, verify in Keychain
  │  └─ security find-generic-password
  │
  └─ Display results
     ├─ Keychain status
     ├─ Secrets count
     ├─ File integrity
     └─ Next steps
```

### 3. Rules Engine Module (`src/rules/mod.rs`)

**Responsibility**: Dynamic security rule management with 3 layers

#### Architecture (3-Layer System)

```
RuleEngine
  │
  ├─ Layer 1: Built-in Rules (Rust source code)
  │  ├─ 30+ hardcoded security patterns
  │  ├─ Core protection rules
  │  └─ Always active
  │
  ├─ Layer 2: Configuration File Rules (~/.keychain/rules.json)
  │  ├─ User-defined JSON rules
  │  ├─ Enable/disable toggle
  │  └─ No recompilation needed
  │
  └─ Layer 3: Environment Variable Rules ($KEYCHAIN_CUSTOM_RULES)
     ├─ Temporary test rules
     ├─ Pipe-separated patterns
     └─ Runtime-only (no persistence)
```

#### Core Data Structures

```rust
pub struct Rule {
    pub id: String,              // Unique identifier
    pub rule_type: RuleType,     // Matching type
    pub description: String,     // Human-readable description
    pub enabled: bool,           // Enable/disable flag
}

pub enum RuleType {
    Substring { pattern: String },           // Simple substring match
    ContainsAll { patterns: Vec<String> },   // All patterns required
    ContainsAny { patterns: Vec<String> },   // Any pattern matches
}

pub struct RuleEngine {
    rules: Vec<Rule>,            // All loaded rules
}
```

#### Rule Matching Algorithm (Turing Option 2 - Hybrid)

**Decision**: Hybrid approach balancing precision vs maintainability

**Algorithm**:
```
1. Iterate through all enabled rules
2. For each rule:
   ├─ Substring rule:
   │  └─ Check if pattern exists in command (case-insensitive)
   ├─ ContainsAll rule:
   │  └─ Check if ALL patterns exist in command
   └─ ContainsAny rule:
      └─ Check if ANY pattern exists in command
3. Return true (blocked) if any rule matches
4. Return false (safe) if no rules match
```

**Why not full regex?**
- Regex adds compilation overhead (slower load times)
- More false positives/negatives if poorly written
- Simple substring matching is fast and maintainable
- 80/20 rule: handles 80% of cases with 20% complexity

#### Rule Loading Sequence

```
RuleEngine::new()
  │
  ├─ 1. Load built-in rules (L1)
  │     └─ 30+ hardcoded patterns in Rust
  │
  ├─ 2. Load config file rules (L2)
  │     ├─ Read ~/.keychain/rules.json
  │     ├─ Parse JSON
  │     └─ Add to rules vector
  │
  └─ 3. Load environment variable rules (L3)
        ├─ Read $KEYCHAIN_CUSTOM_RULES
        ├─ Split by pipe (|)
        └─ Create substring rules dynamically
```

#### Built-in Rules Examples (L1)

| Rule ID | Pattern | Type | Purpose |
|---------|---------|------|---------|
| env_access | .env | substring | Block .env file access |
| docker_config | docker compose config | substring | Block docker secret exposure |
| grep_password | grep PASSWORD | contains_all | Block grep for passwords |
| ssh_access | ~/.ssh | substring | Block SSH key access |
| aws_creds | ~/.aws | substring | Block AWS config access |

### 4. Keychain Module (`src/keychain/mod.rs`)

**Responsibility**: macOS Keychain interaction via security command

```rust
pub struct Keychain {
    service_name: String,
}

impl Keychain {
    pub fn add_secret(&self, key: &str, value: &str) -> Result<()>
    pub fn get_secret(&self, key: &str) -> Result<String>
    pub fn delete_secret(&self, key: &str) -> Result<()>
    pub fn list_secrets(&self) -> Result<Vec<String>>
}
```

**Operations**:
1. **Add**: `security add-generic-password -s <service> -a <key> -w <value>`
2. **Get**: `security find-generic-password -s <service> -a <key> -w`
3. **Delete**: `security delete-generic-password -s <service> -a <key>`
4. **List**: `security dump-keychain` (parsed for service)

**Security Features**:
- Uses system `security` command (no external dependencies)
- Secrets stored in encrypted Keychain database
- Biometric authentication protection
- No secrets in memory after retrieval (returned immediately)

### 5. Configuration Module (`src/config.rs`)

**Responsibility**: Configuration file management

```rust
pub struct Config {
    state_file_path: PathBuf,
    rules_file_path: PathBuf,
    service_name: String,
}

impl Config {
    pub fn get_state_file(&self) -> PathBuf
    pub fn get_rules_file(&self) -> PathBuf
    pub fn ensure_directories(&self) -> Result<()>
}
```

**Key Paths**:
- State file: `~/.keychain/<service-name>.keys`
- Rules file: `~/.keychain/rules.json`
- Log file: `~/.keychain/keychain-cli.log`

### 6. Error Handling Module (`src/error.rs`)

**Responsibility**: Custom error types and handling

```rust
pub enum KeychainError {
    KeychainAccessFailed(String),
    FileNotFound(String),
    InvalidFormat(String),
    ConfigurationError(String),
    ValidationFailed(String),
}

impl Display for KeychainError {
    // User-friendly error messages
}
```

## Data Flow Diagrams

### Setup Flow

```
User Input (.env file)
    │
    ▼
┌─────────────────────────┐
│  setup command          │
│  - Read file            │
│  - Parse KEY=VALUE      │
│  - Filter sensitive     │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Keychain Storage       │
│  - security add-generic │
│  - For each key/value   │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  State File             │
│  ~/.keychain/keys       │
│  (key names only)       │
└─────────────────────────┘
```

### Load Flow

```
Load Request (format: bash/json/export)
    │
    ▼
┌──────────────────────────┐
│  load command            │
│  - Read state file       │
│  - Get key list          │
└────────┬─────────────────┘
         │
         ▼
┌──────────────────────────┐
│  Keychain Retrieval      │
│  - For each key:         │
│  - security get-generic  │
└────────┬─────────────────┘
         │
         ▼
┌──────────────────────────┐
│  Format Output           │
│  - bash:   export VAR    │
│  - json:   {"VAR": ...}  │
│  - export: VAR=value     │
└────────┬─────────────────┘
         │
         ▼
    Stdout Output
```

### Validate Flow (Hook)

```
Claude Code runs command
    │
    ▼
┌──────────────────────────┐
│  Hook: Pre-execution     │
│  Pipes command to stdin  │
└────────┬─────────────────┘
         │
         ▼
┌──────────────────────────┐
│  validate command        │
│  - Read stdin            │
│  - Initialize RuleEngine │
└────────┬─────────────────┘
         │
         ▼
┌──────────────────────────┐
│  Check Danger Rules      │
│  - L1: Built-in rules    │
│  - L2: Config rules      │
│  - L3: Env var rules     │
└────────┬─────────────────┘
         │
    ┌────┴────┐
    ▼         ▼
  Exit 0    Exit 2
 (safe)   (blocked)
    │         │
    ▼         ▼
  Execute   Reject
```

### Check Flow

```
Check Request (--verbose)
    │
    ▼
┌──────────────────────────┐
│  check command           │
│  - Verify state file     │
│  - Count secrets         │
│  - Validate Keychain     │
└────────┬─────────────────┘
         │
         ▼
┌──────────────────────────┐
│  Display Status          │
│  - Initialized?          │
│  - Secret count          │
│  - Hook configured?      │
│  - Recommendations       │
└──────────────────────────┘
```

## Integration Points

### 1. Claude Code Hook Integration

**File**: `~/.claude/settings.json`
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

**Flow**:
1. Claude Code detects Bash command
2. Pipes command text to stdin
3. Calls `keychain-cli validate`
4. Checks exit code (0=allow, 2=block)
5. Executes or rejects command

### 2. Shell Profile Integration

**File**: `~/.zshrc` or `~/.bash_profile`
```bash
eval "$(keychain-cli load --format export)"
```

**Effect**:
- On shell startup, loads all secrets as environment variables
- Secrets available to all child processes
- Only in memory, never written to disk

### 3. Configuration File Integration

**File**: `~/.keychain/rules.json`
- Loaded on each `validate` command
- Enables dynamic rule updates without recompilation
- User can edit to customize security behavior

## Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| setup (61 secrets) | ~3s | Writes to Keychain, one per second |
| load (61 secrets) | ~1s | Reads from Keychain in parallel |
| validate (command) | <10ms | Rule matching, no Keychain access |
| check (verbose) | ~2s | Verifies all secrets in Keychain |

## Security Properties

1. **Keychain Encryption**: All values encrypted by OS, biometric protected
2. **Memory-Only**: Secrets never written to disk logs
3. **Rule Validation**: Every command validated before execution
4. **Metadata Separation**: State file contains only key names, not values
5. **Atomic Operations**: Setup and load are atomic (no partial state)
6. **Process Isolation**: Secrets passed via environment variables, isolated per process

## Extension Points

### Adding New Rule Types

To add a new rule matching type:

1. Add variant to `RuleType` enum in `src/rules/mod.rs`
2. Implement matching logic in `matches()` method
3. Update configuration documentation
4. Add unit tests

Example:
```rust
pub enum RuleType {
    Substring { pattern: String },
    ContainsAll { patterns: Vec<String> },
    ContainsAny { patterns: Vec<String> },
    Regex { pattern: String },  // New type
}
```

### Adding New Commands

To add a new CLI command:

1. Create new file in `src/commands/newcommand.rs`
2. Implement command function
3. Add subcommand to `main.rs` argument parser
4. Document in README

### Adding New Configuration Options

To add configuration options:

1. Update `Config` struct in `src/config.rs`
2. Update configuration file path
3. Update documentation
4. Add environment variable support if needed

## Dependencies

### Core Dependencies
- **clap**: Command-line argument parsing
- **serde**: Serialization framework
- **serde_json**: JSON support
- **log**: Logging framework
- **env_logger**: Logging implementation
- **shellexpand**: Path expansion (~/ support)

### Why Rust?
- Type safety: Prevents entire classes of bugs
- Performance: Minimal overhead for security checks
- Static compilation: Single binary, no runtime dependencies
- Memory safety: No buffer overflows, memory leaks
- Security-focused: Built-in defense against common attacks

## Testing Strategy

### Unit Tests
- Rule matching logic
- Configuration parsing
- Command execution

### Integration Tests
- End-to-end setup → load → validate flow
- Keychain interaction
- File I/O

### Manual Testing
- Hook integration with Claude Code
- Shell profile loading
- Configuration file updates

---

**Last Updated**: 2026-02-19
**Architecture Version**: 1.0
